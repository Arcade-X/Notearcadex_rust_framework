// model_adjustments.js

export function adjustModel(model) {
    // Traverse the model to find specific parts and modify them
    model.traverse(function (child) {
        if (child.isMesh && child.name === 'fan') {
            child.material = new THREE.MeshStandardMaterial({ color: 0xff0000 });
        }
    });
}
