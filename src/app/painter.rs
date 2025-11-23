use eframe::egui::{
    self, Pos2, Stroke, Color32, Rect
};

use crate::app::brushes::crystal::StrokeData;
use crate::app::painter::Blot;  // NEW: blot includes props

pub struct CanvasPainter;

impl CanvasPainter {
    pub fn paint_background(painter: &egui::Painter, rect: Rect, canvas_color: Color32) {
        painter.rect_filled(rect, 0.0, canvas_color);
    }

    pub fn paint_strokes(painter: &egui::Painter, strokes: &[StrokeData], brush_size: f32) {
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
        use crate::app::brushes::blotter_props::BlotShape;

        for b in blots {
            let p = &b.props;

            // -----------------------------------------------------------
            // 1. BASE COLOR / MAIN FILL
            // -----------------------------------------------------------
            let alpha = (p.opacity * 255.0) as u8;
            let base = Color32::from_rgba_unmultiplied(
                b.color.r(), b.color.g(), b.color.b(), alpha,
            );

            match p.shape {
                BlotShape::Circle => {
                    painter.circle_filled(b.pos, p.radius, base);
                }

                BlotShape::Square => {
                    let half = p.radius;
                    let rect = Rect::from_center_size(
                        b.pos,
                        egui::vec2(half * 2.0, half * 2.0),
                    );

                    painter.rect_filled(rect, 0.0, base);
                }
            }

            // -----------------------------------------------------------
            // 2. HALO / GLOW
            // -----------------------------------------------------------
            if p.halo_strength > 0.001 {
                let halo_scale = 1.0 + p.halo_offset * 0.35;
                let halo_radius = (p.radius * halo_scale).abs();
                let halo_alpha = (alpha as f32 * p.halo_strength * 0.6) as u8;

                let halo_color = Color32::from_rgba_unmultiplied(
                    b.color.r(), b.color.g(), b.color.b(), halo_alpha,
                );

                let halo_stroke = Stroke::new(
                    p.radius * 0.15 * p.halo_strength,
                    halo_color,
                );

                match p.shape {
                    BlotShape::Circle => {
                        painter.circle_stroke(b.pos, halo_radius, halo_stroke);
                    }

                    BlotShape::Square => {
                        let half = halo_radius;
                        let rect = Rect::from_center_size(
                            b.pos,
                            egui::vec2(half * 2.0, half * 2.0),
                        );

                        painter.rect_stroke(
                            rect,
                            0.0,
                            halo_stroke,
                        );
                    }
                }
            }

            // -----------------------------------------------------------
            // 3. FEATHERING
            // -----------------------------------------------------------
            if p.softness > 0.01 {
                let feather_radius = p.radius * (1.0 + p.softness * 0.25);
                let feather_alpha = (alpha as f32 * p.softness * 0.4) as u8;

                let feather_color = Color32::from_rgba_unmultiplied(
                    b.color.r(), b.color.g(), b.color.b(), feather_alpha,
                );

                let feather_stroke = Stroke::new(
                    p.radius * 0.12 * p.softness,
                    feather_color,
                );

                match p.shape {
                    BlotShape::Circle => {
                        painter.circle_stroke(b.pos, feather_radius, feather_stroke);
                    }

                    BlotShape::Square => {
                        let half = feather_radius;
                        let rect = Rect::from_center_size(
                            b.pos,
                            egui::vec2(half * 2.0, half * 2.0),
                        );

                        painter.rect_stroke(
                            rect,
                            0.0,
                            feather_stroke,
                        );
                    }
                }
            }
        }
    }

    pub fn paint_active_path(painter: &egui::Painter, pts: &[Pos2]) {
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
        _strokes: &[StrokeData],
        _blots: &[Blot],
    ) {
        // reserved for future overlay layers
    }
}
