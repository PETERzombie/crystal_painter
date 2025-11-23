// app/brushes/blotter_props.rs

use eframe::egui::Color32;

#[derive(Clone, Copy, PartialEq)]
pub enum BlotShape {
    Circle,
    Square,
}

#[derive(Clone, Copy)]
pub struct BlotterProps {
    pub radius: f32,
    pub softness: f32,
    pub opacity: f32,

    pub halo_offset: f32,
    pub halo_strength: f32,

    pub deposit_rate: f32,

    pub shape: BlotShape,
}

impl Default for BlotterProps {
    fn default() -> Self {
        Self {
            radius: 40.0,
            softness: 0.2,
            opacity: 0.85,

            halo_offset: 0.0,
            halo_strength: 0.4,

            deposit_rate: 1.0,

            shape: BlotShape::Circle,
        }
    }
}
