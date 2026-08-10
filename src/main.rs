use crate::app::App;

mod app;
mod camera;
mod graphics;

fn main() {
    if let Err(err) = App::run() {
        eprintln!("failed to start app: {err}");
        std::process::exit(1);
    }
}
