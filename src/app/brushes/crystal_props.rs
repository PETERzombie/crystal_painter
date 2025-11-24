// app/brushes/crystal_props.rs

#[derive(Debug, Clone)]
pub struct CrystalProps {
    pub branch_angle: f32,
    pub branch_decay: f32,
    pub min_segment: f32,
    pub thickness: f32,
}

impl Default for CrystalProps {
    fn default() -> Self {
        Self {
            branch_angle: 0.35,
            branch_decay: 0.7,
            min_segment: 6.0,
            thickness: 2.0,
        }
    }
}
