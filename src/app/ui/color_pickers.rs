use eframe::egui;
use crate::app::state::AppState;

pub fn draw(ui: &mut egui::Ui, state: &mut AppState) {
    ui.label("Brush:");
    ui.color_edit_button_srgba(&mut state.current_color);

    ui.separator();

    ui.label("Canvas:");
    ui.color_edit_button_srgba(&mut state.canvas_bg);
}
