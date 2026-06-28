import { useEffect, useState } from 'react';
import {useNavigate, useLocation} from 'react-router-dom';

import { getBlueprintsOfPlayer, getVehicelTypes, buyBlueprintForPlayer} from './api';


function ResearchBlueprints() {
    const navigate = useNavigate();
    const location = useLocation();
    const initialPlayer = location.state?.player;

    const [blueprints, setBlueprints] = useState([]);
    const [currentPlayer, setCurrentPlayer] = useState(initialPlayer);
    const [selectedId, setSelectedId] = useState('');
    const [showVehicelTypes, setShowVehicelTypes] = useState(false);
    const [vehicelTypes, setVehicleTypes] = useState([]);

    useEffect(() => {
        if (!currentPlayer) {
            return;
        }

        const load = async () => {
            try {
                setPlayerBlueprints();
                setVehicleTypes(await getVehicelTypes());
            } catch {
                setBlueprints([]);
            }
        };
        load();
    }, [currentPlayer]);

    async function setPlayerBlueprints() {
        const blueprints = await getBlueprintsOfPlayer(currentPlayer.id);
        setBlueprints(Array.isArray(blueprints) ? blueprints : []);
    }

    async function buyBlueprint(chassisId, chassisPrice) {
        if (currentPlayer.money < chassisPrice) {
            return;
        }
        const player = await buyBlueprintForPlayer(currentPlayer.id, chassisId);
        setCurrentPlayer(player);
    }

    return (
        <div style={{ display: 'flex' }}>
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
                    <button onClick={() => {
                        setShowVehicelTypes(!showVehicelTypes)
                    }}>Buy</button>
                </div>
                <div style={{ marginTop: 10 }}>
                    <button onClick={() => navigate('/main', { state: { player: currentPlayer } })}>Back</button>
                </div>
            </div>
            {showVehicelTypes && (
                <div style={{ marginLeft: 20, minWidth: 200, borderLeft: '1px solid #ccc', paddingLeft: 10 }}>
                    <h3>Vehicle Types</h3>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
                        {vehicelTypes.map((type) => (
                            <div
                                key={type.id}
                                onClick={() => buyBlueprint(type.id, type.price)}
                                style={{
                                    display: 'flex',
                                    flexDirection: 'column',
                                    alignItems: 'center',
                                    padding: '8px',
                                    border: '1px solid #aaa',
                                    borderRadius: 4,
                                    cursor: 'pointer',
                                    userSelect: 'none',
                                }}
                            >
                                <span style={{ fontWeight: 'bold' }}>{type.name}</span>
                                <img
                                    src={require(`./assets/${type.image_url}`)}
                                    alt={type.name}
                                    style={{ width: 64, height: 64, objectFit: 'contain', margin: '4px 0' }}
                                />
                                <span>Price: {type.price}</span>
                            </div>
                        ))}
                    </div>
                </div>
            )}
        </div>
    );
}

export default ResearchBlueprints;
