// app/ui/top_bar.rs
use eframe::egui::{self, Ui};
use crate::app::state::AppState;
use crate::app::ui::{dropdown, color_pickers, swatches, mode_buttons, canvas_color_picker};

/// Render the top toolbar. Public entry used by state.rs
pub fn show(state: &mut AppState, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Properties dropdown
            dropdown::properties_dropdown(ui, state);

            ui.separator();

            // Color pickers
            color_pickers::draw(ui, state);
            canvas_color_picker::draw(ui, state);


            ui.separator();

            // Pause / Grow / Decay buttons
            if ui.button(if state.paused { "Seed" } else { "Freeze" }).clicked() {
                state.paused = !state.paused;
            }
            if ui.button("Grow").clicked() {
                state.growth_speed = 0.35;
                state.auto_grow = true;
                state.paused = false;
            }
            if ui.button("Decay").clicked() {
                state.growth_speed = -0.08;
                state.auto_grow = true;
                state.paused = false;
            }

            ui.separator();

            // Contain toggle
            if ui.button(if state.contain_growth { "Contain: ON" } else { "Contain: OFF" }).clicked() {
                state.contain_growth = !state.contain_growth;
            }

            ui.separator();

            // Destroy + Leave
            if ui.button("Destroy").clicked() {
                state.should_destroy = true;
            }
            if ui.button("Leave").clicked() {
                state.should_exit = true;
            }

            ui.separator();

            // Blot count display (safe access to AppState.blots)
            ui.label(format!("Blots: {}", state.blots.len()));

            ui.separator();

            // Swatches (right aligned area)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                swatches::draw(ui, state);
            });
        });
    });
}
