// app/brushes/drip_props.rs

#[derive(Debug, Clone)]
pub struct DripProps {
    pub gravity: f32,
    pub viscosity: f32,
    pub thickness: f32,
}

impl Default for DripProps {
    fn default() -> Self {
        Self {
            gravity: 1.2,
            viscosity: 0.9,
            thickness: 2.0,
        }
    }
}
