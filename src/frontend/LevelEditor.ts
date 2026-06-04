import React, { useEffect, useRef, useState } from 'react';
import * as Phaser from 'phaser';

// --- 1. SIMPLE PHASER SCENE FOR THE EDITOR ---
class EditorScene extends Phaser.Scene {
  private currentTile: string = 'dirt';

  constructor() {
    super('EditorScene');
  }

  preload() {
    // Load your assets here
    this.load.image('dirt', 'assets/dirt.png');
    this.load.image('grass', 'assets/grass.png');
  }

  create() {
    this.add.text(20, 20, "Phaser Canvas - Draw Area", { color: '#00ff00' });

    // Listen for grid clicks to place tiles
    this.input.on('pointerdown', (pointer: Phaser.Input.Pointer) => {
      // Snapping logic to a 32x32 grid
      const x = Math.floor(pointer.x / 32) * 32;
      const y = Math.floor(pointer.y / 32) * 32;
      
      this.add.image(x, y, this.currentTile).setOrigin(0);
    });

    // Listen for custom events from React
    this.game.events.on('change-tile', (tileKey: string) => {
      this.currentTile = tileKey;
      console.log(`Phaser switched active tile to: ${tileKey}`);
    });
  }
}

// --- 2. MAIN EDITOR COMPONENT ---
export const LevelEditor: React.FC = () => {
  const phaserRef = useRef<HTMLDivElement>(null);
  const gameRef = useRef<Phaser.Game | null>(null);
  const [selectedTile, setSelectedTile] = useState<string>('dirt');

  // Dummy tile list for the selector panel
  const availableTiles = ['dirt', 'grass', 'stone', 'water'];

  useEffect(() => {
    if (!phaserRef.current || gameRef.current) return;

    // Phaser Config constrained to the parent div's size
    const config: Phaser.Types.Core.GameConfig = {
      type: Phaser.AUTO,
      width: '100%',
      height: '100%',
      parent: phaserRef.current, // <-- Links Phaser to the right-side div
      backgroundColor: '#1d1d1d',
      scene: [EditorScene],
      scale: {
        mode: Phaser.Scale.RESIZE, // Automatically handles resizing
        autoCenter: Phaser.Scale.CENTER_BOTH
      }
    };

    gameRef.current = new Phaser.Game(config);

    // Cleanup on component unmount
    return () => {
      if (gameRef.current) {
        gameRef.current.destroy(true);
        gameRef.current = null;
      }
    };
  }, []);

  // Handle tile change from React UI
  const handleTileSelect = (tile: string) => {
    setSelectedTile(tile);
    if (gameRef.current) {
      // Send the event directly into Phaser's global event bus
      gameRef.current.events.emit('change-tile', tile);
    }
  };

  return (
    <div style={styles.container}>
      {/* LEFT PANEL: REACT UI */}
      <aside style={styles.leftPanel}>
        <h2 style={styles.heading}>Tile Selector</h2>
        <p style={{ color: '#aaa' }}>Selected: <strong>{selectedTile}</strong></p>
        <hr style={styles.divider} />
        
        <div style={styles.tileGrid}>
          {availableTiles.map((tile) => (
            <button
              key={tile}
              onClick={() => handleTileSelect(tile)}
              style={{
                ...styles.tileButton,
                border: selectedTile === tile ? '2px solid #00bcff' : '1px solid #444',
                background: selectedTile === tile ? '#2c3e50' : '#222'
              }}
            >
              {tile.toUpperCase()}
            </button>
          ))}
        </div>
      </aside>

      {/* RIGHT PANEL: PHASER CANVAS */}
      <main ref={phaserRef} style={styles.rightCanvas} />
    </div>
  );
};

// --- 3. INLINE STYLES (Feel free to swap with Tailwind CSS) ---
const styles: { [key: string]: React.CSSProperties } = {
  container: {
    display: 'flex',
    width: '100vw',
    height: '100vh',
    overflow: 'hidden',
    fontFamily: 'sans-serif',
    backgroundColor: '#111',
    color: '#fff'
  },
  leftPanel: {
    width: '300px',
    backgroundColor: '#1a1a1a',
    borderRight: '1px solid #333',
    padding: '20px',
    boxSizing: 'border-box',
    display: 'flex',
    flexDirection: 'column'
  },
  rightCanvas: {
    flex: 1,           // Fills up all remaining space to the right
    height: '100%',
    position: 'relative'
  },
  heading: {
    marginTop: 0,
    fontSize: '1.2rem',
    letterSpacing: '1px'
  },
  divider: {
    border: 'none',
    borderBottom: '1px solid #333',
    margin: '15px 0'
  },
  tileGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(2, 1fr)',
    gap: '10px'
  },
  tileButton: {
    padding: '15px 10px',
    color: '#fff',
    borderRadius: '4px',
    cursor: 'pointer',
    fontSize: '0.8rem',
    fontWeight: 'bold',
    transition: 'all 0.2s ease'
  }
};