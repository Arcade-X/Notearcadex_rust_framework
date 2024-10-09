// animations.js

const mixers = []; // Array to store animation mixers
const clock = new THREE.Clock(); // Clock to keep track of time

export function setupAnimations(gltf, scene) {
    const model = gltf.scene;t 
    scene.add(model);
    console.log('Model loaded and added to the scene.');

    // If the model has animations, setup an animation mixer
    if (gltf.animations.length) {
        const mixer = new THREE.AnimationMixer(model);
        gltf.animations.forEach((clip) => {
            mixer.clipAction(clip).play();
        });
        mixers.push(mixer); // Store the mixer for updating
    }
}

// Update all animation mixers
export function updateAnimations() {
    const delta = clock.getDelta(); // Get the time elapsed since the last frame
    mixers.forEach(mixer => mixer.update(delta));
}

// Main animation loop
export function animateScene(renderer, scene, camera, controls) {
    function animate() {
        requestAnimationFrame(animate);
        updateAnimations();
        controls.update();
        renderer.render(scene, camera);
    }
    animate();
}
