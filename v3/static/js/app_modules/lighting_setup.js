// lighting.js

export function setupLighting(scene) {
    // Add a directional light to simulate sunlight
    const directionalLight = new THREE.DirectionalLight(0xffffff, 1);
    directionalLight.position.set(-9, 15, -10);
    directionalLight.castShadow = true;
    directionalLight.shadow.mapSize.width = 512;
    directionalLight.shadow.mapSize.height = 512;
    scene.add(directionalLight);

    // Add a helper to visualize the directional light (sun)
    const directionalLightHelper = new THREE.DirectionalLightHelper(directionalLight, 5); // 5 is the size of the helper, adjust as needed
    scene.add(directionalLightHelper);

    // Add an ambient light for global illumination
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.2);
    scene.add(ambientLight);

    // Optionally add an environment map using RGBELoader for reflections
    new THREE.RGBELoader()
        .setPath('./hdri/')
        .load('goegap_1k.hdr', function (texture) {
            texture.mapping = THREE.EquirectangularReflectionMapping;
            scene.environment = texture;
        });
}
