import Phaser from 'phaser';
import tilemapImage from '../assets/tiles/tilemap.png';
import playerBaseImage from '../assets/player/base.png';

export default class GameScene extends Phaser.Scene {
    controls;
    playerBase;
    aKey;
    dKey;
    wKey;
    sKey;

    preload ()
    {
        this.load.image('tilemap', tilemapImage);

        this.load.image('player-base', playerBaseImage);
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

        this.playerBase = this.add.sprite(100, 100, 'player-base');
    
        // Keyboard control
        this.aKey = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.A);
        this.dKey = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.D);
        this.wKey = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.W);
        this.sKey = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.S);

        // Camera control
        this.cameras.main.setBounds(0, 0, map.widthInPixels, map.heightInPixels);
        this.cameras.main.startFollow(this.playerBase);
    }

    update (time, delta)
    {
        if (this.aKey.isDown)
        {
            this.playerBase.angle -= 0.4;
        }

        if (this.dKey.isDown)
        {
            this.playerBase.angle += 0.4;
        } 

        let velocity = 0;
        if (this.wKey.isDown)
        {
            velocity = -2;
        }
        if (this.sKey.isDown)
        {
            velocity = 2;
        }

        const velocityX = -Math.cos(this.playerBase.angle / 180 * Math.PI) * velocity;
        const velocityY = -Math.sin(this.playerBase.angle / 180 * Math.PI) * velocity;
        this.playerBase.x += velocityX;
        this.playerBase.y += velocityY;
    }
}
