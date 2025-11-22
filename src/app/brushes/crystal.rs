use eframe::egui::{Painter, Pos2, Stroke, Color32, Vec2};
use crate::app::brushes::crystal_props::CrystalProps;

/// Stroke and segment data used by the painter.
#[derive(Clone)]
pub struct Segment {
    pub start: Pos2,
    pub end: Pos2,
    pub dir: Vec2,
    pub born: std::time::Instant,
    pub generation: u8,
    pub growing: bool,
}

#[derive(Clone)]
pub struct StrokeData {
    pub segments: Vec<Segment>,
    pub color: Color32,
    pub thickness: Option<f32>,
}

impl StrokeData {
    pub fn new(color: Color32, growing: bool) -> Self {
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

pub struct CrystalBrush {
    pub props: CrystalProps,
}

impl CrystalBrush {
    pub fn new() -> Self {
        Self {
            props: CrystalProps::default(),
        }
    }

    /// A simple growth step that extends each segment slightly.
    /// (This is a small, safe implementation suitable for initial integration.)
    pub fn growth_step(&mut self, strokes: &mut Vec<StrokeData>, growth_speed: f32, contain: bool) {
        // Append a small extension to the end of each last segment in each stroke
        for stroke in strokes.iter_mut() {
            if let Some(last) = stroke.segments.last_mut() {
                // compute next end
                let step = growth_speed * 0.5;
                let next = last.end + last.dir * step;
                last.end = next;
            }
        }
    }
}

impl crate::app::brushes::BrushEngine for CrystalBrush {
    fn stroke(&mut self, painter: &Painter, from: Pos2, to: Pos2, color: Color32) {
        // simple immediate render using recursive branches for visual effect
        let dir = to - from;
        let dir_norm = if dir.length_sq() > 0.0 { dir.normalized() } else { dir };
        let seg = StrokeData {
            segments: vec![Segment {
                start: from,
                end: to,
                dir: dir_norm,
                born: std::time::Instant::now(),
                generation: 0,
                growing: true,
            }],
            color,
            thickness: Some(self.props.thickness),
        };

        // painter draw immediate primary segment
        for s in &seg.segments {
            painter.line_segment([s.start, s.end], Stroke::new(self.props.thickness, color));
        }
    }
}
