use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use wgpu::util::DeviceExt;
use nalgebra::{Matrix4, Perspective3, Point3, Vector3}; // For matrix operations

#[wasm_bindgen]
pub async fn compute_heavy_task(canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    console_error_panic_hook::set_once(); // Helps with better error logging

    // Create an instance
    let instance = wgpu::Instance::new(wgpu::Backends::all());

    // Request an adapter
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None, // We will skip surface creation in Rust for now
        force_fallback_adapter: false,
    }).await.ok_or_else(|| JsValue::from_str("Failed to find an appropriate adapter"))?;

    // Request a device and queue
    let device_descriptor = wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
    };
    let (device, queue) = adapter.request_device(&device_descriptor, None)
        .await
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Create a camera (perspective projection)
    let aspect_ratio = canvas.width() as f32 / canvas.height() as f32;
    let camera = Camera::new(aspect_ratio);

    // Set up the lighting
    let ambient_light = AmbientLight::new(0.2); // Ambient light with low intensity
    let sunlight = SunLight::new(Vector3::new(0.0, -1.0, 0.0), [1.0, 1.0, 1.0], 1.0); // Sunlight from top

    // Set up the pipeline and buffers for passing camera and lighting information to shaders

    // The next step would be to bind the camera and lighting data to the shaders
    // through uniform buffers (not included here for simplicity)

    Ok(())
}

// Camera structure to handle view and projection matrices
struct Camera {
    view_matrix: Matrix4<f32>,
    proj_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let eye = Point3::new(0.0, 2.0, 5.0);  // Camera position (adjust as needed)
        let target = Point3::origin();          // Looking at the origin
        let up = Vector3::y();                  // Up direction

        let view_matrix = Matrix4::look_at_rh(&eye, &target, &up);
        let proj_matrix = Perspective3::new(aspect_ratio, 45.0_f32.to_radians(), 0.1, 100.0).to_homogeneous();

        Self {
            view_matrix,
            proj_matrix,
        }
    }
}

// Ambient light structure
struct AmbientLight {
    intensity: f32,
}

impl AmbientLight {
    pub fn new(intensity: f32) -> Self {
        Self { intensity }
    }
}

// Directional sunlight structure
struct SunLight {
    direction: Vector3<f32>,
    color: [f32; 3],
    intensity: f32,
}

impl SunLight {
    pub fn new(direction: Vector3<f32>, color: [f32; 3], intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }
}