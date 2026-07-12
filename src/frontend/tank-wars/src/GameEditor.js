import { useEffect, useRef, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

import Phaser from 'phaser';
import GameEditorScene from './GameEditorScene';
import tilemapImage from './assets/tiles/tilemap.png';

import { getEnemies, getMapData, getUnusedVehiclesOfPlayer, putMapDataOfPlayer } from './api';

const TILE_SIZE = 64;
const VEHICLE_PLACED_EVENT = 'vehicle-placed-event';

function GameEditor() {
    const gameContainerRef = useRef(null);
    const gameRef = useRef(null);

    const [playerVehicles, setPlayerVehicles] = useState([]);
    const [selectedTileId, setSelectedTileId] = useState(0);
    const [selectedVehicleId, setSelectedVehicleId] = useState(null);
    const [tiles, setTiles] = useState([]);

    const { state } = useLocation();
    const map = state?.map;
    const player = state?.player;
    const navigate = useNavigate();

    // Load tile images from the tilemap image
    useEffect(() => {
        let cancelled = false;

        const source = new Image();
        source.src = tilemapImage;

        getUnusedVehiclesOfPlayer(player.id).then(setPlayerVehicles);

        source.onload = () => {
            if (cancelled) {
                return;
            }

            const columns = Math.floor
            (source.width / TILE_SIZE);
            const rows = Math.floor(source.height / TILE_SIZE);
            const extractedTiles = [];
            const playerVehicles = [];

            for (let row = 0; row < rows; row += 1) {
                for (let column = 0; column < columns; column += 1) {
                    const canvas = document.createElement('canvas');
                    canvas.width = TILE_SIZE;
                    canvas.height = TILE_SIZE;

                    const context = canvas.getContext('2d');
                    if (!context) {
                        continue;
                    }

                    context.drawImage(
                        source,
                        column * TILE_SIZE,
                        row * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        0,
                        0,
                        TILE_SIZE,
                        TILE_SIZE
                    );

                    const index = row * columns + column;
                    extractedTiles.push({
                        id: index,
                        name: `Tile ${index}`,
                        src: canvas.toDataURL('image/png')
                    });
                }
            }

            setTiles(extractedTiles);
            setSelectedTileId((previous) => (previous < extractedTiles.length ? previous : 0));
        };

        source.onerror = () => {
            if (!cancelled) {
                setTiles([]);
            }
        };

        return () => {
            cancelled = true;
        };
    }, []);
    
    useEffect(() => {
        if (!gameRef.current) {
            return;
        }

        gameRef.current.registry.set('selectedTileId', Number(selectedTileId));
    }, [selectedTileId]);

    useEffect(() => {
        if (!gameRef.current) {
            return;
        }

        gameRef.current.registry.set('selectedVehicleId', selectedVehicleId);
    }, [selectedVehicleId]);

    // Initialize Phaser game when the component mounts
    useEffect(() => {
        if (!gameContainerRef.current) {
            return undefined;
        }

        const game = new Phaser.Game({
            type: Phaser.AUTO,
            width: 1028,
            height: 768,
            parent: gameContainerRef.current,
            backgroundColor: '#ffffff',
            scene: GameEditorScene,
        });

        gameRef.current = game;
        gameRef.current.registry.set('selectedTileId', Number(selectedTileId));
        gameRef.current.registry.set('map', map);
        gameRef.current.registry.set('playerId', player?.id);

        // Register for vehicle placed in the Game EditorScene and update the playerVehicles state when a vehicle is placed
        const onVehiclePlaced = (placedVehicle) => {
            console.log('Vehicle placed event received in GameEditor:', placedVehicle);
            setSelectedVehicleId(-1);
            setPlayerVehicles((prevVehicles) => prevVehicles.filter((vehicle) => vehicle.id !== placedVehicle.vehicleId));
        }

        let sceneRef = null;
        let registered = false;

        function registerVehiclePlacedListener() {
            if (registered) {
                return;
            }

            sceneRef = gameRef.current.scene.scenes[0];
            if (sceneRef) {
                sceneRef.events.on(VEHICLE_PLACED_EVENT, onVehiclePlaced);
                registered = true;
            }
        }

        // Call directly in case the scene is already created
        registerVehiclePlacedListener();

        // Also call when the scene is created
        gameRef.current.events.on('ready', registerVehiclePlacedListener);
        
        return () => {
            if (sceneRef) {
                sceneRef.events.off(VEHICLE_PLACED_EVENT, onVehiclePlaced);
            }
            if (gameRef.current) {
                gameRef.current.events.off('ready', registerVehiclePlacedListener);
                gameRef.current = null;
            }
            game.destroy(true);
        };
    }, []);

    async function handleSaveMap() {
        if (!player?.id) {
            console.error('player is not set', player);
            return;
        }
        const mapData = gameRef.current.scene.scenes[0].getMapData();
        console.log('Saving map for player', player.id, 'mapData length:', mapData.length);

        putMapDataOfPlayer(player.id, mapData);

    }

    function handleExit() {
        navigate('/main', { state: { player } });
    }

    return (
        <div className="root">
            <main className="canvas-area">
                <div ref={gameContainerRef} className="canvas-container" />
            </main>

            <aside className="control-panel">
                <section className="tile-section">
                    <h2 className="tile-section-title">Tile Images</h2>
                    <div className="tile-grid">
                        {tiles.map((tile) => (
                            <button
                                key={tile.id}
                                type="button"
                                className={`tile-button${selectedTileId === tile.id ? ' tile-button-selected' : ''}`}
                                onClick={() => {
                                    setSelectedTileId(tile.id); 
                                    setSelectedVehicleId(-1);
                                }}
                            >
                                <img src={tile.src} alt={tile.name} className="tile-image" />
                                <span className="tile-name">{tile.name}</span>
                            </button>
                        ))}
                    </div>
                    <div>
                        <button onClick={handleSaveMap}>Save Map</button>
                        <button onClick={handleExit}>Exit</button>
                    </div>
                    <h2 className="tile-section-title">Vehicles</h2>
                    <div className="tile-grid">
                        {playerVehicles.map((vehicle) => (
                            <button
                                key={vehicle.id}
                                type="button"
                                className={`tile-button${selectedVehicleId === vehicle.id ? ' tile-button-selected' : ''}`}
                                onClick={() => {
                                    setSelectedVehicleId(vehicle.id);
                                    setSelectedTileId(-1);
                                }}
                            >
                                <img src={require(`./assets/${vehicle.game_image_url}`)} alt={vehicle.name} className="tile-image" />
                                <span className="tile-name">{vehicle.name}</span>
                            </button>
                        ))}
                    </div>

                </section>
            </aside>
        </div>
    );
}

export default GameEditor;
