// app/ui/swatches.rs
use eframe::egui::{self, Color32, Ui};
use crate::app::state::AppState;

/// Draw swatches panel used in the top bar (compact).
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        //
        // Add swatch button
        //
        if ui.small_button("+").clicked() {
            if !state.swatches.contains(&state.current_color)
                && state.swatches.len() < 32
            {
                state.swatches.push(state.current_color);
                state.selected_swatch = Some(state.swatches.len() - 1);
            }
        }

        //
        // Remove swatch button
        //
        if ui.small_button("-").clicked() {
            if let Some(_) = state.swatches.pop() {
                if let Some(sel) = state.selected_swatch {
                    if sel >= state.swatches.len() {
                        state.selected_swatch = None;
                    }
                }
            }
        }

        //
        // Iterate over swatch indices REVERSED (no immutable borrow!)
        //
        let count = state.swatches.len();
        for idx in (0..count).rev() {
            let col = state.swatches[idx];

            let (rect, resp) = ui.allocate_exact_size(
                egui::vec2(20.0, 20.0),
                egui::Sense::click(),
            );

            ui.painter().rect_filled(rect, 3.0, col);

            // Highlight if selected
            let is_selected = state.selected_swatch == Some(idx);
            if is_selected {
                ui.painter().rect_stroke(
                    rect.expand(2.0),
                    4.0,
                    egui::Stroke::new(2.0, Color32::WHITE),
                    egui::StrokeKind::Middle,
                );
            }

            // Click = select
            if resp.clicked() {
                state.current_color = col;
                state.selected_swatch = Some(idx);
            }

            // Right-click = overwrite swatch
            if resp.secondary_clicked() {
                state.swatches[idx] = state.current_color;
            }
        }
    });
}
