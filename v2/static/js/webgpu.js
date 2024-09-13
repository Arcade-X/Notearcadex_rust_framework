export async function startGPU() {
    if (!navigator.gpu) {
        console.error("WebGPU not supported");
        return;
    }

    const adapter = await navigator.gpu.requestAdapter();
    const device = await adapter.requestDevice();

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

    // Change the clearValue to black (or transparent if preferred)
    const renderPassDescriptor = {
        colorAttachments: [{
            view: context.getCurrentTexture().createView(),
            loadOp: 'clear',
            clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, // Black color
            storeOp: 'store',
        }]
    };

    const commandEncoder = device.createCommandEncoder();
    const passEncoder = commandEncoder.beginRenderPass(renderPassDescriptor);

    passEncoder.end();
    device.queue.submit([commandEncoder.finish()]);
}