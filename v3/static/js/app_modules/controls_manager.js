export function initControls(camera, renderer, scene) {
    const controls = new THREE.OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;
    controls.screenSpacePanning = false;
    controls.maxPolarAngle = Math.PI / 2;

    // Add Axes Helper to the scene
    const axesHelper = new THREE.AxesHelper(10); // Length of the axes lines
    scene.add(axesHelper);

    return controls;
}
