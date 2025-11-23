use eframe::egui::{self, Color32};
use crate::app::brushes::BrushKind;
use crate::app::state::PaintState;

pub fn draw(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        let modes = [
            (BrushKind::Crystal, "Crystal"),
            (BrushKind::Drip, "Drip"),
            (BrushKind::Blotter, "Blotter"),
        ];

        for (mode, label) in modes {
            let selected = state.active_brush == mode;
            let button = if selected {
                egui::Button::new(label).fill(Color32::from_rgb(80, 80, 100))
            } else {
                egui::Button::new(label)
            };

            if ui.add(button).clicked() {
                state.active_brush = mode;
            }
        }
    });
}
