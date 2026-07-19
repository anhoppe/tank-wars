const BASE_URL = 'http://localhost:3001/api';

export async function buyBlueprintForPlayer(playerId, chassisId) {
    const response = await fetch(`${BASE_URL}/blueprints/${playerId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ componentDefinitionId: chassisId }),
    });
    if (!response.ok) {
        throw new Error(`Failed to buy blueprint for player ${playerId}`);
    }
    return await response.json();
}

export async function buyVehicleForPlayer(playerId, blueprintId) {
    const response = await fetch(`${BASE_URL}/fleet/${playerId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ blueprintId: blueprintId }),
    });
    if (!response.ok) {
        throw new Error(`Failed to buy vehicle for player ${playerId}`);
    }
    return await response.json();
}

export async function getAvailableExtensionComponentsByKind(kind) {
    const response = await fetch(`${BASE_URL}/vehicle-types/${kind}`);
    if (!response.ok) {
        throw new Error(`Failed to fetch available component definitions for kind ${kind}`);
    }
    return await response.json();
}

export async function getAvailableMountPointsForBlueprint(blueprintId) {
    const response = await fetch(`${BASE_URL}/blueprints/${blueprintId}/available-mount-points`);
    if (!response.ok) {
        throw new Error(`Failed to fetch available mount points for blueprint ${blueprintId}`);
    }
    return await response.json();
}

export async function getBlueprintsOfPlayer(playerId) {
    const response = await fetch(`${BASE_URL}/blueprints/${playerId}`);
    if (!response.ok) {
        throw new Error(`Failed to fetch blueprints for player ${playerId}`);
    }
    return await response.json();
}

export async function getEnemies(playerId) {
    return fetch(`${BASE_URL}/enemies/${playerId}`)
    .then(res => {
        if (!res.ok) throw new Error(`Server error: ${res.status}`);
        return res.json();
    })
    .catch(err => console.error('Failed to load opponents:', err));
}

export async function getFleetOfPlayer(playerId) {
    const response = await fetch(`${BASE_URL}/fleet/${playerId}`);
    if (!response.ok) {
        throw new Error(`Failed to fetch fleet for player ${playerId}`);
    }
    return await response.json();
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

export async function getVehicleById(vehicleId) {
    const response = await fetch(`${BASE_URL}/vehicle/${vehicleId}`);
    if (!response.ok) {
        throw new Error(`Failed to fetch vehicle with ID ${vehicleId}`);
    }
    return await response.json();
}

export async function getUnusedVehiclesOfPlayer(playerId) {
    const response = await fetch(`${BASE_URL}/map/${playerId}/vehicles/unused`);
    if (!response.ok) {
        throw new Error(`Failed to fetch unused vehicles for player ${playerId}`);
    }
    return await response.json();
}

export async function getVehiclesOnMap(playerId) {
    const response = await fetch(`${BASE_URL}/map/${playerId}/vehicles`);
    if (!response.ok) {
        throw new Error(`Failed to fetch vehicles on map for player ${playerId}`);
    }
    return await response.json();
}

export async function getVehicleTypes() {
    const response = await fetch(`${BASE_URL}/vehicle-types`);
    if (!response.ok) {
        throw new Error(`Failed to fetch vehicle types: ${response.status}`);
    }
    return await response.json();
}

export async function putMapDataOfPlayer(playerId, mapData) {
    const response = await fetch(`${BASE_URL}/map/${playerId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mapData }),
    });
    if (!response.ok) {
        throw new Error(`Failed to save map data for player ${playerId}`);
    }
}

export async function placeVehicleOnMap(playerId, vehicleId, x, y) {
    const response = await fetch(`${BASE_URL}/map/${playerId}/vehicles`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ vehicleId: vehicleId, x: x, y: y }),

    });
    if (!response.ok) {
        throw new Error(`Failed to place vehicle on map for player ${playerId}`);
    }
}
