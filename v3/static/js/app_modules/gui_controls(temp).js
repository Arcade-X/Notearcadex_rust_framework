export function setupGUI(camera, cameraName) {
    const gui = new dat.GUI();
    const cameraFolder = gui.addFolder(`${cameraName} Camera`);

    const cameraSettings = {
        x: camera.position.x,
        y: camera.position.y,
        z: camera.position.z,
        lookAtX: 0,
        lookAtY: 0,
        lookAtZ: 0
    };

    cameraFolder.add(cameraSettings, 'x', -100, 100).onChange(() => updateCamera(camera, cameraSettings));
    cameraFolder.add(cameraSettings, 'y', -100, 100).onChange(() => updateCamera(camera, cameraSettings));
    cameraFolder.add(cameraSettings, 'z', -100, 100).onChange(() => updateCamera(camera, cameraSettings));
    cameraFolder.add(cameraSettings, 'lookAtX', -100, 100).onChange(() => updateCamera(camera, cameraSettings));
    cameraFolder.add(cameraSettings, 'lookAtY', -100, 100).onChange(() => updateCamera(camera, cameraSettings));
    cameraFolder.add(cameraSettings, 'lookAtZ', -100, 100).onChange(() => updateCamera(camera, cameraSettings));
    cameraFolder.open();

    function updateCamera(camera, settings) {
        camera.position.set(settings.x, settings.y, settings.z);
        camera.lookAt(settings.lookAtX, settings.lookAtY, settings.lookAtZ);
    }
}
