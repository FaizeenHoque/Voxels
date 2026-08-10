mod app;
mod camera;
mod mesh;
mod state;
mod vertex;

use crate::app::App;

fn main() {
    if let Err(err) = App::run() {
        eprintln!("failed to start app: {err}");
        std::process::exit(1);
    }
}
