export function initCamera(container) {
    const camera = new THREE.PerspectiveCamera(60, container.clientWidth / container.clientHeight, 0.1, 1000);
    camera.position.set(0, 25, 4);
    camera.rotation.x = 5;
    return camera;
}
