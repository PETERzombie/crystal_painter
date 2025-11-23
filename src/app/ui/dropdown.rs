use eframe::egui::{Ui, Slider, ComboBox};
use crate::app::brushes::BrushKind;
use crate::app::state::PaintState;

use crate::app::brushes::crystal_props::CrystalProps;
use crate::app::brushes::drip_props::DripProps;
use crate::app::brushes::blotter_props::{BlotterProps, BlotShape};

/// The always-visible dropdown controller.
pub fn properties_dropdown(ui: &mut Ui, state: &mut AppState) {
    ui.menu_button("Properties", |ui| {
        ui.heading(match state.active_brush {
            BrushKind::Crystal => "Crystal Properties",
            BrushKind::Drip => "Drip Properties",
            BrushKind::Blotter => "Blotter Properties",
        });

        ui.separator();

        match state.active_brush {

            // ----------------------------
            // CRYSTAL BRUSH PROPERTIES
            // ----------------------------
            BrushKind::Crystal => {
                let props: &mut CrystalProps = &mut state.crystal.props;
                ui.label("Crystal Brush Settings");

                ui.add(Slider::new(&mut props.thickness, 0.5..=6.0).text("Thickness"));
                ui.add(Slider::new(&mut props.branch_angle, 0.1..=1.2).text("Branch Angle"));
                ui.add(Slider::new(&mut props.branch_decay, 0.4..=0.95).text("Branch Decay"));
                ui.add(Slider::new(&mut props.min_segment, 2.0..=20.0).text("Minimum Segment"));
            }

            // ----------------------------
            // DRIP BRUSH PROPERTIES
            // ----------------------------
            BrushKind::Drip => {
                let props: &mut DripProps = &mut state.drip.props;
                ui.label("Drip Brush Settings");

                ui.add(Slider::new(&mut props.thickness, 0.5..=8.0).text("Thickness"));
                ui.add(Slider::new(&mut props.gravity, 0.1..=4.0).text("Gravity"));
                ui.add(Slider::new(&mut props.viscosity, 0.5..=1.0).text("Viscosity"));
            }

            // ----------------------------
            // BLOTTER BRUSH PROPERTIES
            // ----------------------------
            BrushKind::Blotter => {
                let props: &mut BlotterProps = &mut state.blotter.props;
                ui.label("Blotter Brush Settings");

                ui.add(Slider::new(&mut props.radius, 5.0..=150.0).text("Radius"));
                ui.add(Slider::new(&mut props.softness, 0.0..=1.0).text("Softness"));
                ui.add(Slider::new(&mut props.opacity, 0.0..=1.0).text("Opacity"));

                ui.separator();

                ui.label("Halo Offset");
                ui.add(Slider::new(&mut props.halo_offset, -1.0..=1.0).text("Offset"));

                ui.label("Halo Strength");
                ui.add(Slider::new(&mut props.halo_strength, 0.0..=1.0).text("Intensity"));

                ui.label("Deposit Rate");
                ui.add(Slider::new(&mut props.deposit_rate, 0.1..=4.0).text("Speed"));

                ui.separator();

                // Shape dropdown — clean and correct
                ComboBox::from_label("Shape")
                    .selected_text(match props.shape {
                        BlotShape::Circle => "Circle",
                        BlotShape::Square => "Square",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut props.shape, BlotShape::Circle, "Circle");
                        ui.selectable_value(&mut props.shape, BlotShape::Square, "Square");
                    });
            }
        }
    });
}
