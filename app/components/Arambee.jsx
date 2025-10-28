'use client'
import React, { useRef, useEffect, useImperativeHandle, forwardRef } from 'react'
import { useGLTF, useAnimations } from '@react-three/drei'

export const Armabee = forwardRef(function Armabee(props, ref) {
  const group = useRef()
  const { nodes, materials, animations } = useGLTF('./models/Armabee.gltf')
  const { actions } = useAnimations(animations, group)

  useImperativeHandle(ref, () => ({
    playAnimation: (animationName) => {
      if (actions[animationName]) {
        // Stop all current animations
        Object.values(actions).forEach(action => action.stop())
        
        // Play the requested animation once
        const action = actions[animationName]
        action.reset().fadeIn(0.2).setLoop(false).play()
        
        // Return to Flying_Idle after animation completes
        action.onComplete = () => {
          const idleAnimation = actions['Flying_Idle'] || actions['Idle'] || actions['idle'] || Object.values(actions)[0]
          if (idleAnimation && idleAnimation !== action) {
            idleAnimation.reset().fadeIn(0.5).play()
          }
        }
      }
    },
    getAvailableAnimations: () => Object.keys(actions),
    actions
  }))

  useEffect(() => {
    // Play Flying_Idle animation by default
    if (actions && Object.keys(actions).length > 0) {
      const idleAnimation = actions['Flying_Idle']
      if (idleAnimation) {
        idleAnimation.play()
      }
      console.log('Armabee actions:', actions)
    }
  }, [actions])

  return (
    <group ref={group} {...props} dispose={null}>
      <group name="Scene">
        <group name="CharacterArmature">
          <skinnedMesh
            name="Armabee"
            geometry={nodes.Armabee.geometry}
            material={materials.Atlas}
            skeleton={nodes.Armabee.skeleton}
          />
          <primitive object={nodes.Root} />
        </group>
      </group>
    </group>
  )
})

useGLTF.preload('./models/Armabee.gltf')
