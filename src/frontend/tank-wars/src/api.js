const BASE_URL = 'http://localhost:3001/api';

export async function getEnemies(playerId) {
    return fetch(`${BASE_URL}/enemies/${playerId}`)
    .then(res => {
        if (!res.ok) throw new Error(`Server error: ${res.status}`);
        return res.json();
    })
    .catch(err => console.error('Failed to load opponents:', err));
}

export async function getMapData(playerId) {
    const response = await fetch(`${BASE_URL}/map/${playerId}`);
    if (!response.ok) {
        throw new Error(`Failed to fetch map for player ${playerId}`);
    }
    return await response.json();
}

export async function getOrCreatePlayer(name) {
  const response = await fetch(`${BASE_URL}/player`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name }),
  });
  if (!response.ok) {
    throw new Error(`Server error: ${response.status}`);
  }
  return response.json();
}

export async function putMapDataOfPlayer(playerId, mapData) {
    const response = await fetch(`${BASE_URL}/map/${playerId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mapData }),
    });
    console.log('Save map response:', response.status);
}
