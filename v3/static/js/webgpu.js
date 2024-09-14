export async function startGPU() {
    if (!navigator.gpu) {
        console.error("WebGPU not supported");
        return;
    }

    // Initialize WebGPU adapter and device
    const adapter = await navigator.gpu.requestAdapter();
    const device = await adapter.requestDevice();

    // Get canvas and configure the WebGPU context
    const canvas = document.getElementById('sandbox-canvas');
    if (!canvas) {
        console.error('Canvas element not found!');
        return;
    }

    const context = canvas.getContext('webgpu');
    if (!context) {
        console.error('WebGPU context not available');
        return;
    }

    const format = navigator.gpu.getPreferredCanvasFormat();
    context.configure({
        device: device,
        format: format,
    });

    // Set up camera and lighting
    setupCameraAndLighting(device, context);

    // Start rendering loop (you can adjust this as needed)
    renderFrame(device, context);
}

// Set up camera and lighting
function setupCameraAndLighting(device, context) {
    // Set camera values (position, perspective, etc.)
    const cameraPosition = { x: 0.0, y: 2.0, z: 5.0 }; // Front-facing camera
    const sunPosition = { x: 0.0, y: 10.0, z: 0.0 }; // Sun from above
    const globalLightIntensity = 0.8; // Global ambient light intensity

    // Logic for camera and lighting setup can go here, you can pass these
    // values to your shaders, or use them to set up uniform buffers.
    console.log('Camera position:', cameraPosition);
    console.log('Sun position:', sunPosition);
    console.log('Global light intensity:', globalLightIntensity);
}

// Render a frame with WebGPU (basic setup)
function renderFrame(device, context) {
    const renderPassDescriptor = {
        colorAttachments: [{
            view: context.getCurrentTexture().createView(),
            loadOp: 'clear',
            clearValue: { r: 255.0, g: 0.0, b: 0.0, a: 1.0 }, // Clear with black color
            storeOp: 'store',
        }]
    };

    // Create a command encoder and render pass
    const commandEncoder = device.createCommandEncoder();
    const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor);

    // End the render pass and submit the command
    passEncoder.end();
    device.queue.submit([commandEncoder.finish()]);

    console.log('Frame rendered');
}