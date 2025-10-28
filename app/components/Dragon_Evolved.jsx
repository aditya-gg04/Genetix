'use client'
import React, { useRef, useEffect, useImperativeHandle, forwardRef } from 'react'
import { useGLTF, useAnimations } from '@react-three/drei'

export const Dragon_Evolved = forwardRef(function Dragon_Evolved(props, ref) {
    const group = useRef()
    const { nodes, materials, animations } = useGLTF('./models/Dragon_Evolved.gltf')
    const { actions } = useAnimations(animations, group)

    useImperativeHandle(ref, () => ({
        playAnimation: (animationName) => {
            if (actions[animationName]) {
                // Stop all current animations
                Object.values(actions).forEach(action => action.stop())

                // Play the requested animation once
                const action = actions[animationName]
                action.reset()
                    .fadeIn(0.2)
                    .setLoop(1)
                    .play()

                action.clampWhenFinished = true
                action.onComplete = () => {
                    const idleAnimation = actions['Flying_Idle'] || actions['Idle'] || actions['idle']
                    if (idleAnimation && idleAnimation !== action) {
                        action.fadeOut(0.2)
                        idleAnimation.reset().fadeIn(0.2).play()
                    }
                }
            }
        },
        getAvailableAnimations: () => Object.keys(actions),
        actions
    }))

    useEffect(() => {
        if (actions && Object.keys(actions).length > 0) {
            const idleAnimation = actions['Flying_Idle'] || actions['Idle'] || actions['idle']
            if (idleAnimation) {
                idleAnimation.play()
            }
            console.log('Dragon_Evolved actions:', actions)
        }
    }, [actions])

    return (
        <group ref={group} {...props} dispose={null}>
            <group name="Scene">
                <group name="CharacterArmature">
                    <skinnedMesh
                        name="Dragon"
                        geometry={nodes.Dragon.geometry}
                        material={materials.Atlas}
                        skeleton={nodes.Dragon.skeleton}
                    />
                    <primitive object={nodes.Root} />
                </group>
            </group>
        </group>
    )
})

useGLTF.preload('./models/Dragon_Evolved.gltf')