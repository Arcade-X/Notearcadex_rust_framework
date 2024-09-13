import init, { WebGPURenderer } from './pkg/your_wasm_package.js'; // Import the WASM module

window.onload = async function () {
    connectWebSocket(); // Connect WebSocket globally
    await init(); // Initialize the WASM module
};

window.reinitializePage = async function () {
    const currentPath = window.location.pathname;

    if (currentPath === '/sandbox') {
        console.log('On sandbox page');

        const canvas = document.getElementById('sandbox-canvas');
        const startButton = document.getElementById('start-webgpu-btn');

        if (canvas && startButton) {
            let renderer;

            startButton.addEventListener('click', async function () {
                console.log('Start WebGPU button clicked');
                if (!renderer) {
                    renderer = await WebGPURenderer.new('sandbox-canvas');
                }
                renderer.setup_scene(); // Call the scene setup function in Rust
            });
        } else {
            console.error('Canvas or start button not found!');
        }
    } else {
        console.log('Not on sandbox page, skipping WebGPU initialization');
    }
};

// Handle back/forward navigation (pop state)
window.onpopstate = function (event) {
    console.log('Pop state detected, reloading page for: ' + window.location.pathname);
    reinitializePage(); // Reinitialize the page based on the current path
};