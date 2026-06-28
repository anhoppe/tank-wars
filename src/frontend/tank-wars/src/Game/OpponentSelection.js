import { useState, useEffect } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';
import { getEnemies, getMapData } from '../api';

function OpponentSelection() {
    const navigate = useNavigate();
    const location = useLocation();
    const player = location.state?.player;

    const [opponents, setOpponents] = useState([]);
    const [selectedOpponent, setSelectedOpponent] = useState(null);

    useEffect(() => {
        if (!player?.id) return;
        getEnemies(player.id).then(setOpponents);
    }, [player?.id]);

    async function handleStart() {
        let mapData = await getMapData(selectedOpponent.id);
        navigate('/game', { state: { player, mapData: mapData } });
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

            </ul>

            <div>
                <button onClick={handleStart} disabled={!selectedOpponent}>
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
