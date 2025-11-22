pub mod crystal;
pub mod crystal_props;
pub mod drip;
pub mod drip_props;
pub mod blotter;
pub mod blotter_props;

use eframe::egui::{Painter, Pos2, Color32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushKind {
    Crystal,
    Drip,
    Blotter,
}

pub trait BrushEngine {
    fn stroke(&mut self, painter: &Painter, from: Pos2, to: Pos2, color: Color32);
    fn finish_stroke(&mut self) {}
}
