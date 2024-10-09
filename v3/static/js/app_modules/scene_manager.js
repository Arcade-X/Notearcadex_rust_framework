// app.js

import { loadModel } from './model_loader.js';

// Function to load all models required for the app
export function loadAppModels(scene) {
    // Load the floor model
    loadModel('./models/Floor_array.glb', { x: -10, y: -0.1, z: 0.7 }, { x: -0.2, y: 0, z: 0 }, { x: 1, y: 1, z: 1 }, scene);

    // Positions for each GPU model
    const positions = [
        { x: -9, y: 1.1, z: 0.15 },
        { x: -7.5, y: 1.1, z: 0.15 },
        { x: -6, y: 1.1, z: 0.15 },
        { x: -4.5, y: 1.1, z: 0.15 },
        { x: -3, y: 1.1, z: 0.15 },
        { x: -1.5, y: 1.1, z: 0.15 },
        { x: -0, y: 1.1, z: 0.15 },
        { x: 1.5, y: 1.1, z: 0.15 },
        { x: 3, y: 1.1, z: 0.15 },
        { x: 4.5, y: 1.1, z: 0.15 },
        { x: 6, y: 1.1, z: 0.15 },
        { x: 7.5, y: 1.1, z: 0.15 }
    ];

    // Load the GPU model at each specified position
    positions.forEach(position => {
        loadModel('./models/gpu.glb', position, { x: -0.2, y: 0, z: -1.6 }, { x: 1.8, y: 1.8, z: 1.8 }, scene);
    });
}
