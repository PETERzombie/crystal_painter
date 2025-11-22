use eframe::egui::{Ui, Slider};
use crate::app::brushes::BrushKind;
use crate::app::state::AppState;
use crate::app::brushes::crystal_props::CrystalProps;
use crate::app::brushes::drip_props::DripProps;
use crate::app::brushes::blotter_props::BlotterProps;

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
            BrushKind::Crystal => {
                let props: &mut CrystalProps = &mut state.crystal.props;
                ui.label("Crystal Brush Settings");
                ui.add(Slider::new(&mut props.thickness, 0.5..=6.0).text("Thickness"));
                ui.add(Slider::new(&mut props.branch_angle, 0.1..=1.2).text("Branch Angle"));
                ui.add(Slider::new(&mut props.branch_decay, 0.4..=0.95).text("Branch Decay"));
                ui.add(Slider::new(&mut props.min_segment, 2.0..=20.0).text("Minimum Segment"));
            }
            BrushKind::Drip => {
                let props: &mut DripProps = &mut state.drip.props;
                ui.label("Drip Brush Settings");
                ui.add(Slider::new(&mut props.thickness, 0.5..=8.0).text("Thickness"));
                ui.add(Slider::new(&mut props.gravity, 0.1..=4.0).text("Gravity"));
                ui.add(Slider::new(&mut props.viscosity, 0.5..=1.0).text("Viscosity"));
            }
            BrushKind::Blotter => {
                let props: &mut BlotterProps = &mut state.blotter.props;
                ui.label("Blotter Brush Settings");
                ui.add(Slider::new(&mut props.radius, 5.0..=150.0).text("Radius"));
                ui.add(Slider::new(&mut props.softness, 0.0..=1.0).text("Softness"));
            }
        }
    });
}
