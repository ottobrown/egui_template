use eframe::{egui, epi};
use egui::Context;
use epi::Frame;

use crate::state::State;

pub fn render(_state: &mut State, ctx: &Context, _frame: &Frame) {
    egui::CentralPanel::default().show(ctx, |ui| ui.label("HI"));
}
