// app/brushes/mod.rs

pub mod crystal;
pub mod crystal_props;
pub mod drip;
pub mod drip_props;
pub mod blotter;
pub mod blotter_props;

use eframe::egui::{Color32, Painter, Pos2};

/// All brush types available in the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushKind {
    Crystal,
    Drip,
    Blotter,
}

/// Shared interface for all brushes.
///
/// Each brush receives:
/// - `painter`: the egui painter
/// - `from` / `to`: pointer segment positions
/// - `color`: user-selected paint color
pub trait BrushEngine {
    fn stroke(&mut self, painter: &Painter, from: Pos2, to: Pos2, color: Color32);

    /// Optional callback invoked when the pointer is released.
    fn finish_stroke(&mut self) {}
}
