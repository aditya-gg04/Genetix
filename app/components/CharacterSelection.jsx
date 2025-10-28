"use client"
import React from 'react'

export function CharacterSelection({ onSelect, selectedCharacter }) {
    const characters = [
        { id: 'armabee', name: 'Armabee', level: 20, type: 'Flying', image: '/characters/armabee.png' },
        { id: 'bluedemon', name: 'BlueDemon', level: 22, type: 'Dark', image: '/characters/bluedemon.png' },
        { id: 'evoldragon', name: 'Dragon Evolved', level: 25, type: 'Fire', image: '/characters/evoldragon.png' }
    ]

    const getCharacterName = (id) => {
        const character = characters.find(char => char.id === id);
        return character ? character.name : '';
    }

    return (
        <div className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 flex items-center justify-center">
            <div className="max-w-4xl w-full p-8">
                <h1 className="text-4xl font-bold text-white text-center mb-8">Choose Your Character</h1>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {characters.map((character) => (
                        <button
                            key={character.id}
                            onClick={() => onSelect(character.id)}
                            className={`group backdrop-blur-md ${selectedCharacter === character.id
                                    ? 'bg-white/20 border-emerald-400'
                                    : 'bg-white/10 border-white/20'
                                } border-2 rounded-2xl p-6 hover:bg-white/15 transition-all duration-300 transform hover:scale-105`}
                        >
                            <div className="aspect-square rounded-lg mb-4 overflow-hidden bg-white/5">
                                <img
                                    src={character.image}
                                    alt={character.name}
                                    className="w-full h-full object-cover"
                                />
                            </div>
                            <h3 className="text-2xl font-bold text-white mb-2">{character.name}</h3>
                            <div className="flex justify-between">
                                <span className="text-emerald-400">Level {character.level}</span>
                                <span className="text-blue-400">{character.type}</span>
                            </div>
                        </button>
                    ))}
                </div>
                {selectedCharacter && (
                    <div className="mt-8 text-center">
                        <p className="text-white text-lg font-bold">
                            Selected: <span className="text-emerald-400">
                                {getCharacterName(selectedCharacter)}
                            </span>
                        </p>
                    </div>
                )}
            </div>
        </div>
    )
}