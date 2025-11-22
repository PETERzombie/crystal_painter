use eframe::egui::{Painter, Pos2, Color32, Stroke};
use crate::app::brushes::blotter_props::BlotterProps;

#[derive(Clone)]
pub struct Blot {
    pub pos: Pos2,
    pub radius: f32,
    pub color: Color32,
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

impl crate::app::brushes::BrushEngine for BlotterBrush {
    fn stroke(&mut self, painter: &Painter, _from: Pos2, to: Pos2, color: Color32) {
        let radius = self.props.radius;
        let soft = self.props.softness;

        // simple radial strokes that approximate a soft blot
        for i in 0..12 {
            let ang = (i as f32 / 12.0) * std::f32::consts::TAU;
            let edge = Pos2 { x: to.x + ang.cos() * radius, y: to.y + ang.sin() * radius };
            painter.line_segment([to, edge], Stroke::new(1.5 * (1.0 - soft), color));
        }
    }
}
