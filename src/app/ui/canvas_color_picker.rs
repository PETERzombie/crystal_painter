use eframe::egui::{self, Ui};
use crate::app::state::AppState;

/// Simple background color picker block.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    ui.group(|ui| {
        ui.label("Canvas Color");

        let mut color = state.canvas_bg;

        if ui.color_edit_button_srgba(&mut color).changed() {
            state.canvas_bg = color;
        }
    });
}
