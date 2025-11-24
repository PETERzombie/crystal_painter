// app/brushes/crystal.rs

use eframe::egui::{Painter, Pos2, Stroke, Color32, Vec2};
use crate::app::brushes::crystal_props::CrystalProps;

/// A single crystal segment.
#[derive(Clone)]
pub struct Segment {
    pub start: Pos2,
    pub end: Pos2,
    pub dir: Vec2,
    pub born: std::time::Instant,
    pub generation: u8,
    pub growing: bool,
}

/// A stroke consisting of one or more connected crystal segments.
#[derive(Clone)]
pub struct StrokeData {
    pub segments: Vec<Segment>,
    pub color: Color32,
    pub thickness: Option<f32>,
}

impl StrokeData {
    pub fn new(color: Color32, _growing: bool) -> Self {
        Self {
            segments: Vec::new(),
            color,
            thickness: None,
        }
    }

    pub fn add_segment(&mut self, start: Pos2, end: Pos2, dir: Vec2) {
        self.segments.push(Segment {
            start,
            end,
            dir,
            born: std::time::Instant::now(),
            generation: 0,
            growing: true,
        });
    }
}

/// The main crystal brush engine.
pub struct CrystalBrush {
    pub props: CrystalProps,
}

impl CrystalBrush {
    pub fn new() -> Self {
        Self {
            props: CrystalProps::default(),
        }
    }

    /// Very simple linear growth: extend the endpoint of each segment along its direction.
    pub fn growth_step(&mut self, strokes: &mut Vec<StrokeData>, speed: f32, _contain: bool) {
        for stroke in strokes.iter_mut() {
            if let Some(last) = stroke.segments.last_mut() {
                let step = speed * 0.5;
                last.end = last.end + last.dir * step;
            }
        }
    }
}

impl crate::app::brushes::BrushEngine for CrystalBrush {
    fn stroke(&mut self, painter: &Painter, from: Pos2, to: Pos2, color: Color32) {
        let dv = to - from;

        let dir = if dv.length_sq() > 0.0001 {
            dv.normalized()
        } else {
            Vec2::new(1.0, 0.0)
        };

        // immediate one-segment stroke (preview style)
        painter.line_segment(
            [from, to],
            Stroke::new(self.props.thickness, color),
        );
    }
}
