import { useState, useEffect } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';
import { buyVehicleForPlayer, getBlueprintsOfPlayer, getFleetOfPlayer } from './api';

function Fleet() {
    const navigate = useNavigate();
    const location = useLocation();
    const initialPlayer = location.state?.player;

    const [blueprints, setBlueprints] = useState([]);
    const [currentPlayer, setCurrentPlayer] = useState(initialPlayer);
    const [fleet, setFleet] = useState([]);
    const [selectedBlueprintId, setSelectedBlueprintId] = useState('');

    useEffect(() => {
        if (!currentPlayer) {
            return;
        }
        getFleetOfPlayer(currentPlayer.id).then(setFleet).catch((error) => {
            console.error('Failed to load fleet:', error);
        });
        getBlueprintsOfPlayer(currentPlayer.id).then(setBlueprints).catch((error) => {
            console.error('Failed to load blueprints:', error);
        });
    }, [currentPlayer]);

    async function buyVehicle() {
        if (!selectedBlueprintId) {
            return;
        }

        buyVehicleForPlayer(currentPlayer.id, selectedBlueprintId)
        .then((updatedPlayer) => {
            setCurrentPlayer(updatedPlayer);
        })
        .catch((error) => {
            console.error('Failed to buy vehicle:', error);
        });
    }

    return (
        <div>
            <div>
                <h1>Fleet</h1>

                <h2>Vehicles</h2>
                <ul>
                {fleet.map((vehicle) => (
                    <li key={vehicle.id}>
                        <p>Vehicle: {vehicle.image_url}</p>
                    </li>
                ))}
                </ul>

                <h2>Blueprints</h2>
                <ul>
                    {blueprints.map((blueprint) => (
                        <li key={blueprint.id}
                            onClick={() => setSelectedBlueprintId(blueprint.id)}
                            style={{
                                cursor: 'pointer',
                                fontWeight: selectedBlueprintId === blueprint.id ? 'bold' : 'normal',
                            }}
                        >
                            <p>{blueprint.name} - {blueprint.buying_price}</p>
                        </li>
                    ))}
                </ul>
            </div>
            <div>
                <button onClick={buyVehicle} disabled={!selectedBlueprintId}>
                    Buy Vehicle
                </button>
                <button onClick={() => navigate('/main', { state: { player: currentPlayer } })}>
                    Back to Main
                </button>
            </div>
        </div>
    );
}


export default Fleet;