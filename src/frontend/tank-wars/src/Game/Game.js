import Phaser from 'phaser';
import { useEffect, useRef, useState } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';
import GameScene from './GameScene';


function Game() {
    const navigate = useNavigate();
    const location = useLocation();
    const player = location.state?.player;
    const mapData = location.state?.mapData;
    const vehicle = location.state?.vehicle;

    const gameContainerRef = useRef(null);
    const gameRef = useRef(null);


    useEffect(() => {
        if (!gameContainerRef.current) {
            return;
        }

        const game = new Phaser.Game({
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

        gameRef.current = game;
        gameRef.current.registry.set('map', mapData);
        gameRef.current.registry.set('leaveGame', leaveGame);
        gameRef.current.registry.set('vehicle', vehicle);
        gameRef.current.registry.set('player', player);

        return () => {
            gameRef.current = null; 
            game.destroy(true);
        };
    }, []);

    async function leaveGame() {
        navigate('/main', { state: { player } });
    }

    return (
        <div ref={gameContainerRef}/>
    );
}


export default Game;
