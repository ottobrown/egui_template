use eframe::{egui, epi};
use epi::Frame;
use egui::Context;

use crate::app::App;

pub fn render(app: &mut App, ctx: &Context, _frame: &Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("HI")
    });
}