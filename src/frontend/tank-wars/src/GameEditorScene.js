import Phaser from 'phaser';
import tilemapImage from './assets/tiles/tilemap.png';
import cursorImage from './assets/cursor.png';
import { getVehicleById, getVehiclesOnMap, placeVehicleOnMap } from './api';
import playerBaseImage from './assets/player/base.png';
import playerTruckImage from './assets/player/truck.png';

const VEHICLE_PLACED_EVENT = 'vehicle-placed-event';

export default class GameEditorScene extends Phaser.Scene
{
    controls;
    groundLayer;
    playerId;
    vehicleLayer;
    vehiclesOnMap;

    preload ()
    {
        this.load.image('tilemap', tilemapImage);
        this.load.image('cursor', cursorImage);
        this.load.image('player/base.png', playerBaseImage);
        this.load.image('player/truck.png', playerTruckImage);
    }

    async create ()
    {
        // Map / tile loading
        const player_map = this.registry.get('map');

        this.playerId = this.registry.get('playerId');

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
        
        // Load a blank map with a 32 x 32 px tile size. This is the base tile size. This means that
        // tiles in the map will be placed on a 64 x 64 px grid.
        const map = this.make.tilemap({ width: player_map.width, height: player_map.height, tileWidth: 64, tileHeight: 64 });

        // You can also change the base tile size of map like this:
        // map.setBaseTileSize(32, 32);

        // The current tileset image is 64 x 128, which means two 64 x 64 tiles stacked vertically.
        const tiles = map.addTilesetImage('tilemap', null, 64, 64);

        // Create a layer and populate it from saved map_data (2D array of tile indices)
        this.groundLayer = map.createBlankLayer('layer1', tiles);

        const mapData = JSON.parse(player_map.map_data);
        if (Array.isArray(mapData)) {
            mapData.forEach((row, y) => {
                row.forEach((tileIndex, x) => {
                    this.groundLayer.putTileAt(tileIndex, x, y);
                });
            });
        }

        // Load vehicles on the map
        this.vehiclesOnMap = await getVehiclesOnMap(this.playerId);
        this.vehicleLayer = this.add.layer();
        for (const vehicle of this.vehiclesOnMap) {
            const vehicleImage = this.add.image(vehicle.x * 64 + 32, vehicle.y * 64 + 32, vehicle?.game_image_url);
            vehicleImage.setOrigin(0.5, 0.5);
            this.vehicleLayer.add(vehicleImage);
        }

        //  Create a little 32x32 texture to use to show where the mouse is
        const graphics = this.make.graphics({ x: 0, y: 0, add: false, fillStyle: { color: 0xff00ff, alpha: 1 } });

        graphics.fillRect(0, 0, 64, 64);

        graphics.generateTexture('cursor', 64, 64);

        const highlighted = this.add.image(0, 0, 'cursor').setOrigin(0, 0);

        const hitArea = new Phaser.Geom.Rectangle(0, 0, 64, 64);
        const hitAreaCallback = Phaser.Geom.Rectangle.Contains;

        const hitZoneGroup = this.add.group();

        // Create one interactive zone per cell. Group config alone does not create children.
        for (let y = 0; y < map.width; y += 1)
        {
            for (let x = 0; x < map.height; x += 1)
            {
                const zone = this.add.zone((x * 64) + 32, (y * 64) + 32, 64, 64);
                zone.setInteractive(hitArea, hitAreaCallback);
                hitZoneGroup.add(zone);
            }
        }

        this.input.on('gameobjectover', (pointer, gameObject) =>
        {
            highlighted.setPosition(gameObject.x - 32, gameObject.y - 32);
        });

        this.input.on('pointerdown', function (pointer) {
            const selectedTileId = this.registry.get('selectedTileId');
            const selectedVehicleId = this.registry.get('selectedVehicleId');

            if (selectedTileId !== -1) {
                this.groundLayer.putTileAt(selectedTileId, Math.floor(pointer.worldX / 64), Math.floor(pointer.worldY / 64));
            }
            else if (selectedVehicleId !== -1) {
                const x = Math.floor(pointer.worldX / 64);
                const y = Math.floor(pointer.worldY / 64);

                placeVehicleOnMap(this.playerId, selectedVehicleId, x, y)
                    .then(async () => {
                        let placedVehicle = await getVehicleById(selectedVehicleId);
                        const vehicleImage = this.add.image(x * 64 + 32, y * 64 + 32, placedVehicle?.game_image_url);
                        vehicleImage.setOrigin(0.5, 0.5);
                        this.vehicleLayer.add(vehicleImage);
                        this.events.emit(VEHICLE_PLACED_EVENT, { vehicleId: selectedVehicleId });
                    })
                    .catch((error) => {
                        console.error('Failed to place vehicle on map:', error);
                    });
            }
        }, this);

        const help = this.add.text(16, 16, 'Arrows to scroll', {
            fontSize: '18px',
            padding: { x: 10, y: 5 },
            backgroundColor: '#000000',
            fill: '#ffffff'
        });

        help.setScrollFactor(0);
    }

    getMapData() {
        const width = this.groundLayer.layer.width;
        const height = this.groundLayer.layer.height;
        const result = Array.from({ length: height }, (_, y) =>
            Array.from({ length: width }, (_, x) => {
                const tile = this.groundLayer.getTileAt(x, y);
                return tile ? tile.index : 0;
            })
        );
        return JSON.stringify(result);
    }

    update (time, delta)
    {
        if (this.controls) {
            this.controls.update(delta);
        }
    }
}
