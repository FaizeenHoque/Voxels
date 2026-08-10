use std::sync::Arc;
use winit::{dpi::PhysicalPosition, window::Window};

use crate::{
    camera::{Camera, CameraController, CameraUniform},
    graphics::Renderer,
};

pub struct State {
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_controller: CameraController,
    renderer: Renderer,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1200.0 / 600.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_controller = CameraController::new(0.0005);
        let renderer = Renderer::new(window.clone(), &camera_uniform).await?;

        Ok(Self {
            camera_controller,
            camera,
            camera_uniform,
            renderer,
        })
    }

    // Window/surface lifecycle
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    // Per-frame rendering
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.renderer.render()
    }

    // Per-frame state updates
    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.renderer.update_camera_uniform(&self.camera_uniform);
    }

    pub fn handle_key(&mut self, code: winit::keyboard::KeyCode, is_pressed: bool) -> bool {
        self.camera_controller.handle_key(code, is_pressed)
    }

    // Input handling
    pub fn handle_mouse_moved(&mut self, _position: PhysicalPosition<f64>) {
        // let r = position.x / self.config.width as f64;
        // let g = position.y / self.config.height as f64;

        // self.clear_color = wgpu::Color {
        //     r,
        //     g,
        //     b: 0.5,
        //     a: 1.0,
        // };
    }
}
