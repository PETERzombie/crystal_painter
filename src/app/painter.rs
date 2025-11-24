// app/painter.rs
use eframe::egui::{
    self, Align2, Color32, FontId, Pos2, Rect, Stroke,
};

use crate::app::brushes::crystal::StrokeData;
use crate::app::brushes::blotter::Blot;

/// Global painter for all canvas elements.
pub struct CanvasPainter;

impl CanvasPainter {
    /// Paint solid background.
    pub fn paint_background(
        painter: &egui::Painter,
        rect: Rect,
        canvas_color: Color32,
    ) {
        painter.rect_filled(rect, 0.0, canvas_color);
    }

    /// Paint all crystal-type strokes.
    pub fn paint_strokes(
        painter: &egui::Painter,
        strokes: &[StrokeData],
        base_size: f32,
    ) {
        for stroke in strokes {
            for seg in &stroke.segments {
                painter.line_segment(
                    [seg.start, seg.end],
                    Stroke::new(
                        stroke.thickness.unwrap_or(base_size),
                        stroke.color,
                    ),
                );
            }
        }
    }

    /// Paint blotter circles.
    pub fn paint_blots(painter: &egui::Painter, blots: &[Blot]) {
        for b in blots {
            // Apply opacity
            let alpha = (b.opacity * 255.0) as u8;
            let col = Color32::from_rgba_unmultiplied(
                b.color.r(),
                b.color.g(),
                b.color.b(),
                alpha,
            );

            // ---- SINGLE CIRCLE SHAPE ----
            // Main fill
            painter.circle_filled(b.pos, b.radius, col);

            // Feathered halo if softness > 0.01
            if b.softness > 0.001 {
                let feather_alpha =
                    (alpha as f32 * b.softness * 0.5) as u8;

                painter.circle_stroke(
                    b.pos,
                    b.radius * (1.0 + b.softness * 0.15),
                    Stroke::new(
                        b.radius * 0.25 * b.softness,
                        Color32::from_rgba_unmultiplied(
                            b.color.r(),
                            b.color.g(),
                            b.color.b(),
                            feather_alpha,
                        ),
                    ),
                );
            }
        }
    }

    /// Paint live preview line while dragging.
    pub fn paint_active_path(
        painter: &egui::Painter,
        pts: &[Pos2],
    ) {
        if pts.len() < 2 {
            return;
        }

        for w in pts.windows(2) {
            painter.line_segment([w[0], w[1]], Stroke::new(1.0, Color32::WHITE));
        }
    }

    /// Paint overlay with debug / stats.
    pub fn paint_overlay(
        painter: &egui::Painter,
        rect: Rect,
        strokes: &[StrokeData],
        blots: &[Blot],
    ) {
        
    }
}
