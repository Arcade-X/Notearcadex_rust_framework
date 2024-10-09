console.log("singed");

import { setupLighting } from './app_modules/lighting_setup.js';
import { animateScene, updateAnimations } from './app_modules/animation_manager.js';
import { loadAppModels } from './app_modules/scene_manager.js';
import { setupGUI } from './app_modules/gui_controls(temp).js';

import { initRenderer } from './app_modules/renderer_setup.js';  
import { initCamera } from './app_modules/camera_setup.js'; 
import { initControls } from './app_modules/controls_manager.js'; 


class ArcadexFramework {
    constructor(containerId) {
        // Get the HTML container where the scene will be rendered
        this.container = document.getElementById(containerId);
        if (!this.container) {
            console.error(`Container with ID ${containerId} not found.`);
            return;
        }
        // Initialize the scene, renderer, camera, controls, and models
        this.initScene();
        this.renderer = initRenderer(this.container);  // Use the initRenderer function to initialize the renderer
        this.camera = initCamera(this.container);
        this.controls = initControls(this.camera, this.renderer, this.scene);  // Pass the camera, renderer, and scene to initControls
        this.loadModels();
        // Setup lighting in the scene
        setupLighting(this.scene);
        // Adjust the renderer and camera aspect ratio when the window is resized
        window.addEventListener('resize', () => this.onWindowResize(), false);
        // Start the animation loop
        this.animate();
        console.log(`ArcadexFramework initialized for container: ${containerId}`);

        // Setup GUI for camera adjustments
        setupGUI(this.camera, 'arcadex_framework');
    }

    // Initialize a new Three.js scene
    initScene() {
        this.scene = new THREE.Scene();
    }
    
    // Load the models into the scene
    loadModels() {
        loadAppModels(this.scene);
    }

    // Update the camera and renderer when the window is resized
    onWindowResize() {
        this.camera.aspect = this.container.clientWidth / this.container.clientHeight;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(this.container.clientWidth, this.container.clientHeight);
    }

    // Animation loop to render the scene and update controls
    animate() {
        requestAnimationFrame(() => this.animate());
        this.controls.update();
        this.renderer.render(this.scene, this.camera);
        updateAnimations();
    }
}

// Initialize the ArcadexFramework when the script runs
function init() {
    const app = new ArcadexFramework('threejs-container');
}

init();
