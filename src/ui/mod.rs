use eframe::egui;
use eframe::Frame;
use egui::Context;

use crate::state::State;

pub fn render(_state: &mut State, ctx: &Context, _frame: &Frame) {
    egui::CentralPanel::default().show(ctx, |ui| ui.label("HI"));
}
