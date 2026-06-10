import {useLocation, useNavigate} from 'react-router-dom';
import { getMapData } from './api';
import './styles.css';

function Main() {
    const navigate = useNavigate();
    const { state } = useLocation();
    const player = state?.player;

    async function handleExtendFortification() {
        const map = await getMapData(player.id);
        navigate('/game-editor', { state: { map, player } });
    }

    async function handleResearchBlueprints() {
        navigate('/research-blueprints', { state: { player } });
    }

    return (
        <div>
            <h1>Plan your next move!</h1>
            <div className="main-actions">
                <button className="button" onClick={handleExtendFortification}>Extend Fortification</button>
                <button className="button" onClick={handleResearchBlueprints}>Research Blueprints</button>
                <button className="button" onClick={() => navigate('/opponent-selection', { state: { player } })}>Attack</button>
                <button className="button button-full" onClick={() => navigate('/')}>Exit Game</button>
            </div>
        </div>
    );
}

export default Main;
