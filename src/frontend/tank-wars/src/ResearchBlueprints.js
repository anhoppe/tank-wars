import { useEffect, useState } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';

import { getBlueprintsOfPlayer, getVehicelTypes } from './api';


function ResearchBlueprints() {
    const navigate = useNavigate();
    const location = useLocation();
    const player = location.state?.player;

    const [blueprints, setBlueprints] = useState([]);
    const [vehicelTypes, setVehicleTypes] = useState([]);
    const [selectedId, setSelectedId] = useState('');
    useEffect(() => {
        if (!player) {
            return;
        }

        const load = async () => {
            try {
                const blueprints = await getBlueprintsOfPlayer(player.id);
                setBlueprints(Array.isArray(blueprints) ? blueprints : []);

                setVehicleTypes(await getVehicleTypes());
            } catch {
                setBlueprints([]);
            }
        };
        load();
    }, [player]);

    return (
        <div>
            <h2>Blueprints</h2>
            <select
                size={8}
                value={selectedId}
                onChange={(e) => setSelectedId(e.target.value)}
                style={{ minWidth: 320}}
                >
                    {blueprints.length === 0 ? (
                        <option disabled>No blueprints available</option>
                    ) : (
                        blueprints.map((blueprint) => (
                            <option key={blueprint.id} value={blueprint.id}>
                                {blueprint.name} | Price: {blueprint.buying_price}
                            </option>
                        ))
                    )}
                </select>
                <div style={{ marginTop: 3 }}>
                    <toggle>Buy</toggle>
                </div>
                <div style={{ marginTop: 10 }}>
                    <button onClick={() => navigate('/main', { state: { player } })}>Back</button>
                </div>
        </div>
    );
}

export default ResearchBlueprints;
