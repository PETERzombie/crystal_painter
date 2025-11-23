use eframe::egui::{Painter, Pos2, Color32};
use crate::app::brushes::blotter_props::BlotterProps;
use crate::app::brushes::BrushEngine;

#[derive(Clone)]
pub struct Blot {
    pub pos: Pos2,
    pub radius: f32,
    pub color: Color32,
    pub softness: f32,
    pub opacity: f32,
}

pub struct BlotterBrush {
    pub props: BlotterProps,
}

impl BlotterBrush {
    pub fn new() -> Self {
        Self {
            props: BlotterProps::default(),
        }
    }
}

///
/// The key idea:
/// - A blot = ONE circular mark
/// - Overlapping blots create density
/// - Holding still creates many blots at the same position
///
impl BrushEngine for BlotterBrush {
    fn stroke(&mut self, painter: &Painter, _from: Pos2, to: Pos2, color: Color32) {
        let radius = self.props.radius;
        let opacity = (self.props.opacity * 255.0) as u8;
        let softness = self.props.softness;

        // MAIN circle (like original)
        painter.circle_filled(
            to,
            radius,
            Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                opacity,
            ),
        );

        // Optional VERY light feathering — subtle, not Gaussian
        if softness > 0.01 {
            let feather_alpha = (opacity as f32 * softness * 0.5) as u8;

            painter.circle_stroke(
                to,
                radius * (1.0 + softness * 0.15),
                eframe::egui::Stroke::new(
                    radius * 0.25 * softness,
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), feather_alpha),
                ),
            );
        }
    }
}
