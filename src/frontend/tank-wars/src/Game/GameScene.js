import Phaser from 'phaser';
import tilemapImage from '../assets/tiles/tilemap.png';

export default class GameScene extends Phaser.Scene {
    controls;

    preload ()
    {
        this.load.image('tilemap', tilemapImage);
    }

    create ()
    {
        const enemy_map = this.registry.get('map');
        
        // Load a blank map with a 32 x 32 px tile size. This is the base tile size. This means that
        // tiles in the map will be placed on a 64 x 64 px grid.
        const map = this.make.tilemap({ width: enemy_map.width, height: enemy_map.height, tileWidth: 64, tileHeight: 64 });

        // The current tileset image is 64 x 128, which means two 64 x 64 tiles stacked vertically.
        const tiles = map.addTilesetImage('tilemap', null, 64, 64);

        // Create a layer and populate it from saved map_data (2D array of tile indices)
        this.layer = map.createBlankLayer('layer1', tiles);
        const layer = this.layer;

        const mapData = JSON.parse(enemy_map.map_data);
        if (Array.isArray(mapData)) {
            mapData.forEach((row, y) => {
                row.forEach((tileIndex, x) => {
                    layer.putTileAt(tileIndex, x, y);
                });
            });
        }

        //  Create a little 32x32 texture to use to show where the mouse is
        const graphics = this.make.graphics({ x: 0, y: 0, add: false, fillStyle: { color: 0xff00ff, alpha: 1 } });

        graphics.fillRect(0, 0, 64, 64);

        graphics.generateTexture('cursor', 64, 64);
    
        const cursorKeys = this.input.keyboard.createCursorKeys();

        const controlConfig = {
            camera: this.cameras.main,
            left: cursorKeys.left,
            right: cursorKeys.right,
            up: cursorKeys.up,
            down: cursorKeys.down,
            speed: 0.5
        };

        this.controls = new Phaser.Cameras.Controls.FixedKeyControl(controlConfig);
    }
}
