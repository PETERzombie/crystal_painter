use eframe::egui::{self, Pos2, Stroke, Color32, Rect, FontId, Align2};

use crate::app::brushes::crystal::StrokeData;
use crate::app::brushes::blotter::Blot;

pub struct CanvasPainter;

impl CanvasPainter {
    pub fn paint_background(
        painter: &egui::Painter,
        rect: Rect,
        canvas_color: Color32,
    ) {
        painter.rect_filled(rect, 0.0, canvas_color);
    }

    pub fn paint_strokes(
        painter: &egui::Painter,
        strokes: &[StrokeData],
        brush_size: f32,
    ) {
        for stroke in strokes {
            for seg in &stroke.segments {
                painter.line_segment(
                    [seg.start, seg.end],
                    Stroke::new(brush_size, stroke.color),
                );
            }
        }
    }

    pub fn paint_blots(painter: &egui::Painter, blots: &[Blot]) {
        for b in blots {
            painter.circle_filled(b.pos, b.radius, b.color);
        }
    }

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

    pub fn paint_overlay(
        painter: &egui::Painter,
        rect: Rect,
        strokes: &[StrokeData],
        blots: &[Blot],
    ) {
        let seg_count: usize =
            strokes.iter().map(|s| s.segments.len()).sum();

        let info = format!("Segments: {} | Blots: {}", seg_count, blots.len());

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
