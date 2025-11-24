// app/ui/dropdown.rs
use eframe::egui::{Ui, ComboBox};
use crate::app::brushes::BrushKind;
use crate::app::state::AppState;

/// Small properties dropdown used for brush property selection and similar.
/// This one provides a property menu for the current brush selection.
pub fn properties_dropdown(ui: &mut Ui, state: &mut AppState) {
    ui.menu_button("Properties", |ui| {
        ui.heading(match state.active_brush {
            BrushKind::Crystal => "Crystal Properties",
            BrushKind::Drip => "Drip Properties",
            BrushKind::Blotter => "Blotter Properties",
        });
        ui.separator();

        match state.active_brush {
            BrushKind::Crystal => {
                ui.label("Crystal brush (use top toolbar sliders in future).");
            }
            BrushKind::Drip => {
                ui.label("Drip brush (use top toolbar sliders in future).");
            }
            BrushKind::Blotter => {
                ui.label("Blotter brush (use top toolbar sliders in future).");
                ui.label(format!("Radius: {:.1}",   state.blotter_props.radius));
                ui.label(format!("Softness: {:.2}", state.blotter_props.softness));
                ui.label(format!("Opacity: {:.2}",  state.blotter_props.opacity));
            }
        }
    });

    // Example ComboBox showing brush kinds (useful if you want an explicit selector)
    ui.horizontal(|ui| {
        ui.label("Brush:");
        let labels = ["Crystal", "Drip", "Blotter"];
        let mut sel = match state.active_brush {
            BrushKind::Crystal => 0usize,
            BrushKind::Drip => 1usize,
            BrushKind::Blotter => 2usize,
        };

        ComboBox::from_id_source("brush_kind_combobox")
            .selected_text(labels[sel])
            .show_ui(ui, |ui| {
                for (i, lab) in labels.iter().enumerate() {
                    if ui.selectable_label(sel == i, *lab).clicked() {
                        sel = i;
                    }
                }
            });

        // apply selection (keeps AppState authoritative)
        let new_kind = match sel {
            0 => BrushKind::Crystal,
            1 => BrushKind::Drip,
            _ => BrushKind::Blotter,
        };
        state.active_brush = new_kind;
    });
}
