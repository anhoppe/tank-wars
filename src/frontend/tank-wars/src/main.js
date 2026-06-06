import {useNavigate} from 'react-router-dom';

async function fetchMap(playerId) {
}

function Main() {
    const navigate = useNavigate();
    const { state } = useLocation();
    const player = state?.player;

    async function handleExtendFortification() {
        const map = await fetchMap(player.id);
        navigate('/game-editor', { state: { map } });
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
