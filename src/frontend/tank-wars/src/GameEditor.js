import { useEffect, useRef, useState } from 'react';
import Phaser from 'phaser';
import GameEditorScene from './GameEditorScene';
import tilemapImage from './assets/tiles/tilemap.png';

const TILE_SIZE = 64;

const styles = {
    root: {
        display: 'flex',
        minHeight: '100vh',
        backgroundColor: '#f5f7fb'
    },
    controlPanel: {
        width: '360px',
        minWidth: '360px',
        maxWidth: '360px',
        boxSizing: 'border-box',
        padding: '24px',
        borderLeft: '1px solid #d7deea',
        backgroundColor: '#ffffff'
    },
    title: {
        marginTop: 0,
        marginBottom: '20px'
    },
    controls: {
        display: 'flex',
        flexDirection: 'column',
        gap: '12px'
    },
    button: {
        padding: '10px 14px',
        fontSize: '15px',
        border: '1px solid #cad4e6',
        borderRadius: '8px',
        backgroundColor: '#f8fbff',
        cursor: 'pointer'
    },
    canvasArea: {
        flex: 1,
        padding: '20px',
        boxSizing: 'border-box'
    },
    canvasContainer: {
        width: '100%',
        height: '100%',
        minHeight: '560px',
        border: '2px dashed #bfd0ec',
        borderRadius: '12px',
        backgroundColor: '#eaf1fc'
    },
    tileSection: {
        marginTop: '18px'
    },
    tileSectionTitle: {
        margin: '0 0 10px 0',
        fontSize: '16px'
    },
    tileGrid: {
        display: 'grid',
        gridTemplateColumns: 'repeat(2, minmax(0, 1fr))',
        gap: '10px'
    },
    tileButton: {
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        gap: '8px',
        padding: '10px',
        border: '1px solid #cad4e6',
        borderRadius: '10px',
        backgroundColor: '#f8fbff',
        cursor: 'pointer'
    },
    tileButtonSelected: {
        border: '2px solid #2f6fd1',
        backgroundColor: '#e8f1ff'
    },
    tileImage: {
        width: '100%',
        height: '76px',
        objectFit: 'contain',
        imageRendering: 'pixelated'
    },
    tileName: {
        fontSize: '13px',
        color: '#34496b'
    }
};


function GameEditor() {
    const gameContainerRef = useRef(null);
    const gameRef = useRef(null);

    const [tiles, setTiles] = useState([]);
    const [selectedTileId, setSelectedTileId] = useState(0);

    useEffect(() => {
        let cancelled = false;

        const source = new Image();
        source.src = tilemapImage;

        source.onload = () => {
            if (cancelled) {
                return;
            }

            const columns = Math.floor(source.width / TILE_SIZE);
            const rows = Math.floor(source.height / TILE_SIZE);
            const extractedTiles = [];

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
        
        return () => {
            gameRef.current = null;
            game.destroy(true);
        };
    }, []);

    return (
        <div style={styles.root}>
            <main style={styles.canvasArea}>
                <div ref={gameContainerRef} style={styles.canvasContainer} />
            </main>

            <aside style={styles.controlPanel}>
                <section style={styles.tileSection}>
                    <h2 style={styles.tileSectionTitle}>Tile Images</h2>
                    <div style={styles.tileGrid}>
                        {tiles.map((tile) => (
                            <button
                                key={tile.id}
                                type="button"
                                style={{
                                    ...styles.tileButton,
                                    ...(selectedTileId === tile.id ? styles.tileButtonSelected : {})
                                }}
                                onClick={() => setSelectedTileId(tile.id)}
                            >
                                <img src={tile.src} alt={tile.name} style={styles.tileImage} />
                                <span style={styles.tileName}>{tile.name}</span>
                            </button>
                        ))}
                    </div>
                </section>
            </aside>
        </div>
    );
}

export default GameEditor;
