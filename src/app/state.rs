// app/state.rs

use eframe::egui::{Color32, Pos2};
use crate::app::brushes::{
    BrushEngine,
    blotter::BlotterBrush,
    blotter::Blot,
    blotter_props::BlotterProps,
};

#[derive(PartialEq)]
pub enum ActiveBrush {
    Blotter,
}

pub struct PaintState {
    pub active_brush: ActiveBrush,

    pub blotter: BlotterBrush,
    pub blots: Vec<Blot>,              // ← RESTORED QUEUED BLOTS

    pub color: Color32,
    pub last_pos: Option<Pos2>,
    pub is_drawing: bool,

    // Restored flags
    pub should_destroy: bool,
    pub should_exit: bool,

    pub last_blot_pos: Option<Pos2>,   // ← spacing via deposit_rate
}

impl PaintState {
    pub fn new() -> Self {
        Self {
            active_brush: ActiveBrush::Blotter,

            blotter: BlotterBrush::new(),
            blots: Vec::new(),

            color: Color32::from_rgb(200, 200, 255),

            last_pos: None,
            is_drawing: false,

            last_blot_pos: None,

            should_destroy: false,
            should_exit: false,
        }
    }

    // -------------------------------------------------------------------------
    // Restore queued-blot behavior
    // -------------------------------------------------------------------------

    pub fn push_blot(&mut self, pos: Pos2) {
        let p = &self.blotter.props;

        self.blots.push(Blot {
            pos,
            radius: p.radius,
            softness: p.softness,
            opacity: p.opacity,
            color: self.color,
        });
    }

    pub fn try_deposit_blot(&mut self, pos: Pos2) {
        let rate = self.blotter.props.deposit_rate.max(0.1);

        match self.last_blot_pos {
            None => {
                self.push_blot(pos);
                self.last_blot_pos = Some(pos);
            }
            Some(prev) => {
                if prev.distance(pos) >= rate * 8.0 {
                    self.push_blot(pos);
                    self.last_blot_pos = Some(pos);
                }
            }
        }
    }

    // -------------------------------------------------------------------------
    // Called every pointer movement
    // -------------------------------------------------------------------------

    pub fn paint_stroke(&mut self, pos: Pos2) {
        if !self.is_drawing {
            self.last_pos = Some(pos);
            return;
        }

        match self.active_brush {
            ActiveBrush::Blotter => {
                self.try_deposit_blot(pos);
            }
        }

        self.last_pos = Some(pos);
    }

    // -------------------------------------------------------------------------

    pub fn set_color(&mut self, color: Color32) {
        self.color = color;
    }

    pub fn begin_stroke(&mut self, pos: Pos2) {
        self.is_drawing = true;
        self.last_pos = Some(pos);
        self.last_blot_pos = None;  // fresh spacing
    }

    pub fn end_stroke(&mut self) {
        self.is_drawing = false;
        self.last_pos = None;
        self.last_blot_pos = None;
    }
}
