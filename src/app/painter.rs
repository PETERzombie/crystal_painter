use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, Stroke,
};

use crate::brushes::crystal::StrokeData;
use crate::brushes::blot::Blot;

/// The main renderer for all canvas content.
pub struct CanvasPainter;

impl CanvasPainter {
    /// Paints the solid background of the canvas using the user-selected canvas color.
    pub fn paint_background(
        painter: &egui::Painter,
        rect: Rect,
        canvas_color: Color32,
    ) {
        painter.rect_filled(rect, 0.0, canvas_color);
    }

    /// Paint crystal-style strokes (or any stroke-based brush data structure).
    pub fn paint_strokes(
        painter: &egui::Painter,
        strokes: &[StrokeData],
        base_size: f32,
    ) {
        for stroke in strokes {
            for seg in &stroke.segments {
                painter.line_segment(
                    [seg.start, seg.end],
                    Stroke::new(stroke.thickness.unwrap_or(base_size), stroke.color),
                );
            }
        }
    }

    /// Paint drip or blot-style circles.
    pub fn paint_blots(painter: &egui::Painter, blots: &[Blot]) {
        for b in blots {
            painter.circle_filled(b.pos, b.radius, b.color);
        }
    }

    /// Paint the in-progress line while drawing (white preview).
    pub fn paint_active_path(
        painter: &egui::Painter,
        pts: &[Pos2],
    ) {
        if pts.len() < 2 {
            return;
        }

        for w in pts.windows(2) {
            painter.line_segment(
                [w[0], w[1]],
                Stroke::new(1.0, Color32::WHITE),
            );
        }
    }

    /// Paint debug overlay: segment count, blot count.
    pub fn paint_overlay(
        painter: &egui::Painter,
        rect: Rect,
        strokes: &[StrokeData],
        blots: &[Blot],
    ) {
        let total_segments: usize =
            strokes.iter().map(|s| s.segments.len()).sum();

        let info = format!(
            "Segments: {} | Blots: {}",
            total_segments,
            blots.len()
        );

        let pos = rect.right_top() - egui::vec2(260.0, 6.0);

        painter.text(
            pos,
            Align2::LEFT_TOP,
            info,
            FontId::proportional(12.0),
            Color32::WHITE,
        );
    }
}
