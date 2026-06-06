import {useLocation, useNavigate} from 'react-router-dom';

async function fetchMap(playerId) {
    const response = await fetch(`http://localhost:3001/api/map/${playerId}`);
    if (!response.ok) {
        throw new Error(`Server error: ${response.status}`);
    }

    return response.json();
}

function Main() {
    const navigate = useNavigate();
    const { state } = useLocation();
    const player = state?.player;

    async function handleExtendFortification() {
        const map = await fetchMap(player.id);
        navigate('/game-editor', { state: { map, player } });
    }

    return (
        <div>
            <h1>Plan your next move!</h1>   
            <button onClick={handleExtendFortification}>Extend Fortification</button>
            <button>Attack</button>
        </div>
    );
}

export default Main;
