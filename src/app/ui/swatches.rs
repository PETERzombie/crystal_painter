use eframe::egui::{self, Color32, Stroke, StrokeKind};
use crate::app::state::PaintState;

const MAX_SWATCHES: usize = 10;

pub fn draw(ui: &mut egui::Ui, state: &mut AppState) {
    ui.spacing_mut().item_spacing.x = 8.0;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if state.swatches.len() < MAX_SWATCHES {
            if ui.add_sized([24.0, 20.0], egui::Button::new("+")).clicked() {
                state.swatches.push(state.current_color);
                state.selected_swatch = Some(state.swatches.len() - 1);
            }
        }

        if ui.add_sized([24.0, 20.0], egui::Button::new("-")).clicked() {
            if !state.swatches.is_empty() {
                state.swatches.pop();
                if let Some(sel) = state.selected_swatch {
                    if sel >= state.swatches.len() {
                        state.selected_swatch = None;
                    }
                }
            }
        }

        let len = state.swatches.len();
        for idx in (0..len).rev() {
            let col = state.swatches[idx];
            let is_selected = state.selected_swatch == Some(idx);
            let (rect, resp) = ui.allocate_exact_size(egui::Vec2::splat(20.0), egui::Sense::click());
            let painter = ui.painter();
            painter.rect_filled(rect, 3.0, col);

            if is_selected {
                painter.rect_stroke(
                    rect.expand(2.0),
                    4.0,
                    Stroke::new(2.0, Color32::WHITE),
                    StrokeKind::Outside,
                );
            }

            if resp.clicked() {
                state.current_color = col;
                state.selected_swatch = Some(idx);
            }
            if resp.secondary_clicked() {
                state.swatches[idx] = state.current_color;
            }
        }

        let preview_rect = ui.allocate_exact_size(egui::Vec2::splat(20.0), egui::Sense::hover());
        ui.painter().rect_filled(preview_rect.0, 3.0, state.current_color);
    });
}
