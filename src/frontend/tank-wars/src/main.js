import {useLocation, useNavigate} from 'react-router-dom';
import { getMapData } from './api';

function Main() {
    const navigate = useNavigate();
    const { state } = useLocation();
    const player = state?.player;

    async function handleExtendFortification() {
        const map = await getMapData(player.id);
        navigate('/game-editor', { state: { map, player } });
    }

    return (
        <div>
            <h1>Plan your next move!</h1>   
            <button onClick={handleExtendFortification}>Extend Fortification</button>
            <button onClick={() => navigate('/opponent-selection', { state: { player } })}>Attack</button>
        </div>
    );
}

export default Main;
