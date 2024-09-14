import { startGPU } from './webgpu.js'; // Import the WebGPU logic

// Initialize WebSocket connection on page load
window.onload = function () {
    connectWebSocket(); // Connect WebSocket globally
};

window.reinitializePage = function () {
    const currentPath = window.location.pathname;

    if (currentPath === '/sandbox') {
        console.log('On sandbox page');

        // Check if the sandbox canvas and button exist before proceeding
        const canvas = document.getElementById('sandbox-canvas');
        const startButton = document.getElementById('start-webgpu-btn');

        if (canvas && startButton) {
            startButton.addEventListener('click', function () {
                console.log('Start WebGPU button clicked');
                startGPU(); // Start WebGPU when the button is clicked
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