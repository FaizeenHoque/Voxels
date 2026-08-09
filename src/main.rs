mod app;

use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
use winit::window::Window;

use crate::app::App;

pub struct State {
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self { window })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {}

    pub fn render(&mut self) {
        self.window.request_redraw();
    }
}

fn main() {
    if let Err(err) = App::run() {
        eprintln!("failed to start app: {err}");
        std::process::exit(1);
    }
}
