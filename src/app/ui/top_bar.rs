use eframe::egui;
use crate::app::state::PaintState;
use crate::app::ui::{dropdown, color_pickers, swatches, mode_buttons};

/// Render the top toolbar. This is the public entry used by state.rs
pub fn show(state: &mut PaintState, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Mode buttons
            mode_buttons::draw(ui, state);

            ui.separator();

            // Properties dropdown
            dropdown::properties_dropdown(ui, state);

            ui.separator();

            // Color pickers
            color_pickers::draw(ui, state);

            ui.separator();

            // Pause / Grow / Decay buttons
            if ui.button(if state.paused { "Seed" } else { "Freeze" }).clicked() {
                state.paused = !state.paused;
            }
            if ui.button("Grow").clicked() {
                state.growth_speed = 0.35;
                state.auto_grow = true;
            }
            if ui.button("Decay").clicked() {
                state.growth_speed = -0.08;
                state.auto_grow = true;
            }

            ui.separator();

            // Contain toggle
            if ui.button(if state.contain_growth { "Contain: ON" } else { "Contain: OFF" }).clicked() {
                state.contain_growth = !state.contain_growth;
            }

            ui.separator();

            // -------------------------------------------------------
            // 🔥 Add Destroy + Leave buttons here
            // -------------------------------------------------------
            if ui.button("Destroy").clicked() {
                state.should_destroy = true;
            }

            if ui.button("Leave").clicked() {
                state.should_exit = true;
            }
            ui.separator();
            ui.label(format!("Blots: {}", self.blots.len()));
            ui.separator();
            // -------------------------------------------------------
            
            // Swatches (right aligned)
            swatches::draw(ui, state);
        });
    });
}
