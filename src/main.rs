use eframe::{run_native, NativeOptions};

mod state;
mod ui;

fn main() {
    let state = state::State::new();
    let ops = NativeOptions::default();

    run_native(Box::new(state), ops)
}
