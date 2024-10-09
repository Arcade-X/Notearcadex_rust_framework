// model_init.js

import { setupAnimations } from './animation_manager.js';
import { adjustModel } from './event_manager.js';

export function loadModel(url, position, rotation, scale, scene) {
    const loader = new THREE.GLTFLoader();
    loader.load(url, function (gltf) {
        const model = gltf.scene;
        model.position.set(position.x, position.y, position.z);
        model.rotation.set(rotation.x, rotation.y, rotation.z);
        model.scale.set(scale.x, scale.y, scale.z);

        // Apply model adjustments
        adjustModel(model);

        scene.add(model);
        setupAnimations(gltf, scene);
    }, undefined, function (error) {
        console.error('Error loading model:', error);
    });
}
