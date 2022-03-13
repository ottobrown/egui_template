use eframe::{epi, egui};
use egui::Context;
use epi::Frame;

use crate::ui;

pub struct App {}
impl App {
    pub fn new() -> Self {
        App {}
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        ui::render(self, ctx, frame);
    }

    fn name(&self) -> &str {
        "Egui App"
    }
}