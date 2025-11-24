// src/app/brushes/blotter_props.rs
use serde::{Deserialize, Serialize};

/// User-adjustable parameters for the blotter brush.
/// These map directly to Blot fields and brush behavior.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlotterProps {
    /// Radius of each blot (in points).
    pub radius: f32,

    /// Softness factor: 0.0 = hard edge, 1.0 = softer halo.
    pub softness: f32,

    /// Blot opacity 0.0–1.0.
    pub opacity: f32,

    /// Minimum distance traveled before depositing the next blot.
    pub spacing: f32,
}

impl Default for BlotterProps {
    fn default() -> Self {
        Self {
            radius: 12.0,
            softness: 0.15,
            opacity: 0.9,
            spacing: 4.0, // moderately dense
        }
    }
}
