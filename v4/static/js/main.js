import { connectWebSocket } from './ws_functions.js'; // Import WebSocket logic

// Initialize WebSocket connection and reinitialize page when DOM is fully loaded
document.addEventListener('DOMContentLoaded', function () {
    connectWebSocket(); // Establish WebSocket globally for both login and sandbox
});
