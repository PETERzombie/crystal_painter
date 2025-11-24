// app/brushes/drip.rs
use eframe::egui::{Painter, Pos2, Stroke, Color32};
use crate::app::brushes::drip_props::DripProps;
use crate::app::brushes::BrushEngine;

/// Simple drip brush: draws a falling, gravity-affected line segment.
pub struct DripBrush {
    pub props: DripProps,
    velocity: f32,
}

impl DripBrush {
    pub fn new() -> Self {
        Self {
            props: DripProps::default(),
            velocity: 0.0,
        }
    }

    /// Optional per-frame physics update (currently unused).
    pub fn tick(&mut self) {
        // Placeholder for future drip physics.
        // We keep this for extensibility.
    }
}

impl BrushEngine for DripBrush {
    fn stroke(&mut self, painter: &Painter, from: Pos2, mut to: Pos2, color: Color32) {
        // Gravity increases downward velocity each stroke step.
        self.velocity += self.props.gravity * 0.02;

        // Viscosity dampens motion to avoid extreme acceleration.
        self.velocity *= self.props.viscosity;

        // Apply the vertical drift.
        to.y += self.velocity;

        // Paint the drip segment
        painter.line_segment([from, to], Stroke::new(self.props.thickness, color));
    }

    fn finish_stroke(&mut self) {
        // Reset velocity so the next stroke starts fresh.
        self.velocity = 0.0;
    }
}
