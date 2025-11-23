#[derive(Debug, Clone)]
pub struct BlotterProps {
    pub radius: f32,
    pub softness: f32,
    pub opacity: f32,
}

impl Default for BlotterProps {
    fn default() -> Self {
        Self {
            radius: 35.0,
            softness: 0.4,
            opacity: 0.5,
        }
    }
}
