use eframe::{egui, epi};
use egui::Context;
use epi::Frame;

use crate::ui;

pub struct State {}
impl State {
    pub fn new() -> Self {
        State {}
    }
}

impl epi::App for State {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        ui::render(self, ctx, frame);
    }

    fn name(&self) -> &str {
        "Egui App"
    }
}
