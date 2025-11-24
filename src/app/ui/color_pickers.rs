// src/app/ui/color_pickers.rs
use eframe::egui::{self, Color32, Ui};

use crate::app::state::AppState;

pub fn draw(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Color:");

        let mut col = state.current_color;

        egui::color_picker::color_edit_button_srgba(
            ui,
            &mut col,
            egui::color_picker::Alpha::BlendOrAdditive,
        );

        state.current_color = col;
    });
}
