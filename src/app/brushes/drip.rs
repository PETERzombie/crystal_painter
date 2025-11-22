use eframe::egui::{Painter, Pos2, Stroke, Color32};
use crate::app::brushes::drip_props::DripProps;

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

    /// Called by app when growth/physics tick runs (optional)
    pub fn tick(&mut self) {
        // placeholder; real drip physics go here
    }
}

impl crate::app::brushes::BrushEngine for DripBrush {
    fn stroke(&mut self, painter: &Painter, from: Pos2, mut to: Pos2, color: Color32) {
        // gravity-like effect
        self.velocity += self.props.gravity * 0.02;
        self.velocity *= self.props.viscosity;

        to.y += self.velocity;

        painter.line_segment([from, to], Stroke::new(self.props.thickness, color));
    }

    fn finish_stroke(&mut self) {
        self.velocity = 0.0;
    }
}
