use eframe::{run_native, NativeOptions};

mod app;
mod ui;

fn main() {
    let app = app::App::new();
    let ops = NativeOptions::default();

    run_native(Box::new(app), ops)
}

