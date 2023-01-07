use super::*;

pub struct FlowerStats {
    pub size: f32,
    pub radius: f32,
}

impl FlowerStats {
    pub fn new(size: f32, radius: f32) -> Self {
        Self { size, radius }
    }
}
