import { useEffect } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';

import { getBlueprintsOfPlayer } from './api';


function ResearchBlueprints() {
    const navigate = useNavigate();
    const location = useLocation();
    const player = location.state?.player;

    this.blueprints = [];

    useEffect(() => {
        this.blueprints = getBlueprintsOfPlayer(player.id);
    }, []);

    return (
        <div>
            <p>Research Blueprints Page - Under Construction</p>
            <button onClick={() => navigate('/main', { state: { player } })}>Back</button>
        </div>
    );
}

export default ResearchBlueprints;