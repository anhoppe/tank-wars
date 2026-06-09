import Phaser from 'phaser';
import { useEffect, useRef, useState } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';
import GameScene from './GameScene';


function Game() {
    const navigate = useNavigate();
    const location = useLocation();
    const player = location.state?.player;
    const mapData = location.state?.mapData;

    const gameContainerRef = useRef(null);
    const gameRef = useRef(null);


    useEffect(() => {
        if (!gameContainerRef.current) {
            return;
        }

        const Game = new Phaser.Game({
            type: Phaser.AUTO,
            width: 1280,
            height: 1024,
            parent: gameContainerRef.current,
            physics: {
                default: 'arcade',
                arcade: { debug: true }
            },
            scene: GameScene,
        });

        gameRef.current = Game;
        gameRef.current.registry.set('map', mapData);

        return () => {
            gameRef.current = null;
            Game.destroy(true);
        };
    }, []);

    return (
        <div ref={gameContainerRef}/>
    );
}


export default Game;
