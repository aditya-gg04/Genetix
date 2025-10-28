"use client"
import { useState, useRef, useEffect } from "react"
import { Canvas } from "@react-three/fiber"
import { PerspectiveCamera } from "@react-three/drei"
import { Armabee } from "@/components/Arambee"
import { BlueDemon } from "@/components/BlueDemon"

export default function PokemonBattle() {
  const [playerHP] = useState(65) // Mock data - doesn't change
  const [enemyHP] = useState(100) // Mock data - doesn't change
  const [selectedAction, setSelectedAction] = useState(null)
  const [playerActions, setPlayerActions] = useState([])
  const [isAnimating, setIsAnimating] = useState(false)
  
  const armabeeRef = useRef()
  const blueDemonRef = useRef()

  // Load available animations when models are ready
  useEffect(() => {
    const loadActions = () => {
      // Load player actions
      if (armabeeRef.current && playerActions.length === 0) {
        try {
          const actions = armabeeRef.current.getAvailableAnimations()
          console.log('🐝 Armabee animations available:', actions)
          
          if (actions && actions.length > 0) {
            // Filter out Flying_Idle since it's the default idle animation
            const actionsList = actions
              .filter(action => action !== 'Flying_Idle')
              .map((action, index) => ({
                name: action,
                displayName: action.charAt(0).toUpperCase() + action.slice(1).replace(/([A-Z])/g, ' $1'),
                animation: action
              }))
            setPlayerActions(actionsList)
            console.log('🎮 Armabee actions loaded (excluding Flying_Idle):', actionsList)
            console.log('🐝 All available Armabee animations:', actions)
          } else {
            // Fallback actions
            const fallbackActions = [
              { name: 'Attack1', displayName: 'Attack 1', animation: 'Attack1' },
              { name: 'Attack2', displayName: 'Attack 2', animation: 'Attack2' },
              { name: 'Special', displayName: 'Special', animation: 'Special' },
              { name: 'Idle', displayName: 'Idle', animation: 'Idle' }
            ]
            setPlayerActions(fallbackActions)
            console.log('🔄 Fallback Armabee actions loaded:', fallbackActions)
          }
        } catch (error) {
          console.error('Error loading player actions:', error)
        }
      }
    }

    // Check if models are ready and load actions
    const checkModels = setInterval(() => {
      loadActions()
      
      // Stop checking once actions are loaded
      if (playerActions.length > 0) {
        clearInterval(checkModels)
        console.log('✅ Animation loading complete')
      }
    }, 1000)

    // Initial check after a delay
    setTimeout(loadActions, 2000)

    return () => clearInterval(checkModels)
  }, [])

  const handleAction = async (actionData) => {
    if (isAnimating) return
    
    setIsAnimating(true)
    setSelectedAction(actionData)
    
    console.log(`🎬 Playing animation: ${actionData.animation}`)
    
    // Play player animation
    if (armabeeRef.current) {
      armabeeRef.current.playAnimation(actionData.animation)
    }
    
    // Wait for animation
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    console.log(`✅ Animation complete: ${actionData.displayName}`)
    setIsAnimating(false)
  }

  const hpPercentage = (playerHP / 65) * 100
  const enemyHPPercentage = (enemyHP / 100) * 100

  return (
    <div className="relative w-full h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 overflow-hidden">
      {/* Animated background elements */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute top-0 left-1/4 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl animate-pulse" />
        <div className="absolute bottom-0 right-1/4 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-pulse" />
      </div>

      {/* 3D Battle Scene */}
      <div className="absolute inset-0">
        <Canvas shadows>
          <PerspectiveCamera makeDefault position={[0, 3, 8]} rotation={[-0.25, 0, 0]} fov={50} />
          <ambientLight intensity={0.8} />
          <directionalLight position={[10, 10, 5]} intensity={1.2} castShadow />
          <hemisphereLight intensity={0.6} groundColor="#8B7355" />

          <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, -0.5, 0]} receiveShadow>
            <planeGeometry args={[20, 15]} />
            <meshStandardMaterial color="#90C850" />
          </mesh>

          <mesh rotation={[-Math.PI / 2, 0, 0]} position={[-2.5, -0.48, 2]} receiveShadow>
            <circleGeometry args={[2.2, 32]} />
            <meshStandardMaterial color="#78B850" />
          </mesh>

          <mesh rotation={[-Math.PI / 2, 0, 0]} position={[3, -0.47, -3]} receiveShadow>
            <circleGeometry args={[2.2, 32]} />
            <meshStandardMaterial color="#78B850" />
          </mesh>

          <group position={[-2.5, 0, 2]} rotation={[0, Math.PI * 0.9, 0]}>
            <Armabee ref={armabeeRef} scale={[0.6, 0.6, 0.6]} />
          </group>

          <group position={[3, 0, -3]} rotation={[0, -Math.PI * 0.15, 0]}>
            <BlueDemon ref={blueDemonRef} scale={[0.65, 0.65, 0.65]} />
          </group>
        </Canvas>
      </div>

      {/* Enemy Status Card - Top Left */}
      <div className="absolute top-6 left-6 z-10">
        <div className="backdrop-blur-md bg-white/10 border border-white/20 rounded-2xl p-5 w-80 shadow-2xl hover:bg-white/15 transition-all duration-300">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-white font-bold text-lg">BlueDemon</h3>
              <p className="text-blue-300 text-sm font-semibold">Lv 22</p>
            </div>
            <div className="text-right">
              <p className="text-white/60 text-xs">Enemy</p>
            </div>
          </div>

          {/* HP Bar */}
          <div className="mb-3">
            <div className="flex justify-between items-center mb-2">
              <span className="text-white/70 text-xs font-semibold">HP</span>
              <span className="text-white/70 text-xs">{Math.round(enemyHP)}/100</span>
            </div>
            <div className="relative h-3 bg-white/10 rounded-full overflow-hidden border border-white/20">
              <div
                className="absolute left-0 top-0 h-full transition-all duration-500 rounded-full"
                style={{
                  width: `${enemyHPPercentage}%`,
                  background:
                    enemyHPPercentage > 50
                      ? "linear-gradient(to right, #10b981, #34d399)"
                      : enemyHPPercentage > 20
                        ? "linear-gradient(to right, #f59e0b, #fbbf24)"
                        : "linear-gradient(to right, #ef4444, #f87171)",
                }}
              />
            </div>
          </div>

          {/* Status indicators */}
          <div className="flex gap-2">
            <div className="flex-1 h-1 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-full opacity-60" />
          </div>
        </div>
      </div>

      {/* Player Status Card - Top Right */}
      <div className="absolute top-6 right-6 z-10">
        <div className="backdrop-blur-md bg-white/10 border border-white/20 rounded-2xl p-6 w-96 shadow-2xl hover:bg-white/15 transition-all duration-300">
          <div className="flex items-center justify-between mb-5">
            <div>
              <h3 className="text-white font-bold text-xl">Armabee</h3>
              <p className="text-emerald-300 text-sm font-semibold">Lv 20</p>
            </div>
            <div className="text-right">
              <p className="text-white/60 text-xs">Your Pokémon</p>
            </div>
          </div>

          {/* HP Bar */}
          <div className="mb-4">
            <div className="flex justify-between items-center mb-2">
              <span className="text-white/70 text-xs font-semibold">HP</span>
              <span className="text-white/70 text-xs">{Math.round(playerHP)}/65</span>
            </div>
            <div className="relative h-4 bg-white/10 rounded-full overflow-hidden border border-white/20">
              <div
                className="absolute left-0 top-0 h-full transition-all duration-500 rounded-full"
                style={{
                  width: `${hpPercentage}%`,
                  background:
                    hpPercentage > 50
                      ? "linear-gradient(to right, #10b981, #34d399)"
                      : hpPercentage > 20
                        ? "linear-gradient(to right, #f59e0b, #fbbf24)"
                        : "linear-gradient(to right, #ef4444, #f87171)",
                }}
              />
            </div>
          </div>

          {/* EXP Bar */}
          <div>
            <div className="flex justify-between items-center mb-2">
              <span className="text-white/70 text-xs font-semibold">EXP</span>
              <span className="text-white/70 text-xs">75%</span>
            </div>
            <div className="relative h-2 bg-white/10 rounded-full overflow-hidden border border-white/20">
              <div className="absolute left-0 top-0 h-full w-3/4 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full" />
            </div>
          </div>
        </div>
      </div>

      {/* Animation Status - Top Center */}
      <div className="absolute top-6 left-1/2 transform -translate-x-1/2 z-10">
        <div className="backdrop-blur-md bg-white/10 border border-white/20 rounded-xl p-3 w-80 shadow-xl">
          <p className="text-white/80 text-sm font-medium text-center">
            {isAnimating ? `Playing: ${selectedAction?.displayName || 'Animation'}` : 'Ready to animate'}
          </p>
          <div className="flex justify-center mt-2">
            <div className={`px-3 py-1 rounded-full text-xs font-bold ${
              isAnimating 
                ? 'bg-yellow-500/80 text-white' 
                : 'bg-emerald-500/80 text-white'
            }`}>
              {isAnimating ? '🎬 Animating' : '✅ Ready'}
            </div>
          </div>
        </div>
      </div>

      {/* Action Menu - Bottom */}
      <div className="absolute bottom-0 left-0 right-0 z-20">
        <div className="backdrop-blur-md bg-gradient-to-t from-slate-900/95 to-slate-900/80 border-t border-white/20 px-8 py-6">
          <div className="max-w-6xl mx-auto">
            {/* Dialog Text */}
            <div className="mb-6">
              <p className="text-white text-lg font-bold">
                Choose an <span className="text-emerald-400">Armabee</span> animation to play:
              </p>
              {isAnimating && (
                <p className="text-yellow-400 text-sm mt-2">Playing animation...</p>
              )}
              
              {/* Debug info */}
              <div className="text-xs text-white/50 mt-2">
                Available Actions: {playerActions.length}
              </div>
            </div>

            {/* Action Buttons Grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              {playerActions.length > 0 && playerActions.map((action, index) => {
                const colors = [
                  { bg: "linear-gradient(135deg, #ef4444 0%, #dc2626 100%)", shadow: "rgba(239, 68, 68, 0.3)" },
                  { bg: "linear-gradient(135deg, #f59e0b 0%, #d97706 100%)", shadow: "rgba(245, 158, 11, 0.3)" },
                  { bg: "linear-gradient(135deg, #10b981 0%, #059669 100%)", shadow: "rgba(16, 185, 129, 0.3)" },
                  { bg: "linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%)", shadow: "rgba(59, 130, 246, 0.3)" },
                  { bg: "linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%)", shadow: "rgba(139, 92, 246, 0.3)" },
                  { bg: "linear-gradient(135deg, #ec4899 0%, #db2777 100%)", shadow: "rgba(236, 72, 153, 0.3)" }
                ]
                const colorSet = colors[index % colors.length]
                
                return (
                  <button
                    key={action.name}
                    onClick={() => handleAction(action)}
                    disabled={isAnimating}
                    className="group relative overflow-hidden rounded-xl p-4 font-bold text-white transition-all duration-300 transform hover:scale-105 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                    style={{
                      background: colorSet.bg,
                      boxShadow: `0 8px 16px ${colorSet.shadow}`,
                    }}
                  >
                    <div className="absolute inset-0 bg-white/20 opacity-0 group-hover:opacity-100 transition-opacity" />
                    <span className="relative text-sm md:text-lg">
                      {action.displayName}
                    </span>
                  </button>
                )
              })}
              
              {playerActions.length === 0 && (
                <div className="col-span-2 md:col-span-4 text-center py-8">
                  <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-white"></div>
                  <p className="text-white/70 mt-2">Loading Armabee animations...</p>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
