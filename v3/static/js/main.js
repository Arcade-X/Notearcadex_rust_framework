import { connectWebSocket } from './ws_functions.js'; // Import WebSocket logic

// Initialize WebSocket connection and reinitialize page when DOM is fully loaded
document.addEventListener('DOMContentLoaded', function () {
    connectWebSocket(); // Establish WebSocket globally for both login and sandbox
    reinitializePage(); // Initialize page-specific logic
});

window.reinitializePage = function () {
    const currentPath = window.location.pathname;

    // This is now just checking the path, no Three.js logic here anymore
    if (currentPath === '/sandbox') {
        console.log('On sandbox page');
    } else {
        console.log('Not on sandbox page, skipping Three.js initialization');
    }
};