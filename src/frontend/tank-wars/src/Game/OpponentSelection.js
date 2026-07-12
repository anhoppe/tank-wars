import { useState, useEffect } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';
import { getEnemies, getFleetOfPlayer, getMapData } from '../api';

function OpponentSelection() {
    const navigate = useNavigate();
    const location = useLocation();
    const player = location.state?.player;

    const [opponents, setOpponents] = useState([]);
    const [playerVehicles, setPlayerVehicles] = useState([]);
    const [selectedOpponent, setSelectedOpponent] = useState(null);
    const [selectedVehicle, setSelectedVehicle] = useState(null);
    
    useEffect(() => {
        if (!player?.id) return;
        getEnemies(player.id).then(setOpponents);
        getFleetOfPlayer(player.id).then(setPlayerVehicles);
    }, [player?.id]);

    async function handleStart() {
        let mapData = await getMapData(selectedOpponent.id);
        navigate('/game', { state: { player, mapData: mapData, opponentId: selectedOpponent.id, vehicle: selectedVehicle } });
    }

    return (
        <div>
            <h1>Opponent Selection</h1>
            <p>Player: {player?.name}</p>

            <ul>
                {opponents.map(opponent => (
                    <li
                        key={opponent.id}
                        onClick={() => setSelectedOpponent(opponent)}
                        style={{
                            cursor: 'pointer',
                            fontWeight: selectedOpponent?.id === opponent.id ? 'bold' : 'normal',
                        }}
                    >
                        {opponent.name} — Score: {opponent.score}
                    </li>
                ))}
            </ul>

            <p>Combat Vehicle</p>

            <ul>    
                {playerVehicles.map(vehicle => (
                    <li key={vehicle.id}
                        onClick={() => setSelectedVehicle(vehicle)}
                        style={{
                            cursor: 'pointer',
                            fontWeight: selectedVehicle?.id === vehicle.id ? 'bold' : 'normal',
                        }}
                    >
                        {vehicle.image_url}
                    </li>
                ))}
            </ul>

            <div>
                <button onClick={handleStart} disabled={!selectedOpponent || !selectedVehicle}>
                    Start Game
                </button>
                <button onClick={() => navigate('/main', { state: { player } })}>
                    Back to Main
                </button>
            </div>
        </div>
    );
}

export default OpponentSelection;
