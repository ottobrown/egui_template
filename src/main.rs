use eframe::{run_native, NativeOptions};

mod state;
mod ui;

fn main() {
    let ops = NativeOptions::default();

    run_native(
        "Egui App",
        ops,
        Box::new(|cc| Box::new(state::State::new(cc))),
    )
}
