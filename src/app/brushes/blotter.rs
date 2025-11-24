// src/app/brushes/blotter.rs
use eframe::egui::{Color32, Pos2};
use crate::app::brushes::blotter_props::BlotterProps;

/// A single paint blot placed on the canvas.
#[derive(Clone)]
pub struct Blot {
    pub pos: Pos2,
    pub radius: f32,
    pub color: Color32,
    pub softness: f32,
    pub opacity: f32,
}

/// Tick-based engine that generates blots along the stroke path.
#[derive(Clone)]
pub struct Blotter {
    /// Last point where movement was sampled.
    last_pos: Option<Pos2>,

    /// Distance accumulated since last deposit.
    stroke_accum: f32,
}

impl Blotter {
    pub fn new() -> Self {
        Self {
            last_pos: None,
            stroke_accum: 0.0,
        }
    }

    /// Called when the stroke starts.
    pub fn begin_stroke(&mut self, pos: Pos2) {
        self.last_pos = Some(pos);
        self.stroke_accum = 0.0;
    }

    /// Called when the stroke ends.
    pub fn end_stroke(&mut self) {
        self.last_pos = None;
        self.stroke_accum = 0.0;
    }

    /// Called every frame while the brush is moving.
    /// Returns newly generated blots.
    pub fn tick(
        &mut self,
        current_pos: Pos2,
        props: &BlotterProps,
        current_color: Color32,
    ) -> Vec<Blot> {
        let mut new_blots = Vec::new();

        let Some(last) = self.last_pos else {
            self.last_pos = Some(current_pos);
            return new_blots;
        };

        // distance since last movement sample
        let dx = current_pos.x - last.x;
        let dy = current_pos.y - last.y;
        let dist = (dx * dx + dy * dy).sqrt();

        self.stroke_accum += dist;

        // minimum spacing between blots
        let spacing = props.spacing.max(0.1);

        // deposit blots evenly along the movement segment
        while self.stroke_accum >= spacing {
            self.stroke_accum -= spacing;

            // position along the segment
            let t = 1.0 - (self.stroke_accum / spacing).clamp(0.0, 1.0);
            let blot_x = last.x + dx * t;
            let blot_y = last.y + dy * t;

            new_blots.push(Blot {
                pos: Pos2::new(blot_x, blot_y),
                radius: props.radius,
                color: current_color,
                softness: props.softness,
                opacity: props.opacity,
            });
        }

        self.last_pos = Some(current_pos);

        new_blots
    }
}
