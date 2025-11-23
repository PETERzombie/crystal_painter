use eframe::egui::{Pos2, Color32};
use crate::app::brushes::blotter_props::{BlotterProps, BlotShape};
use crate::app::brushes::BrushEngine;
use crate::app::painter::Blot;

pub struct BlotterBrush {
    pub props: BlotterProps,
}

impl BlotterBrush {
    pub fn new() -> Self {
        Self {
            props: BlotterProps::default(),
        }
    }
}

impl BrushEngine for BlotterBrush {
    /// Instead of painting directly, create a Blot struct.
    fn stroke(&mut self, _from: Pos2, to: Pos2, color: Color32) -> Option<Blot> {
        Some(Blot {
            pos: to,
            color,
            props: self.props.clone(),  // blots store the props used at the time
        })
    }
}
