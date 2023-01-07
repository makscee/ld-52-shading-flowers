use super::*;

mod stats;

pub use stats::*;

#[derive(HasId)]
pub struct Flower {
    pub id: Id,
    pub position: Vec2<f32>,
    pub stats: FlowerStats,
}

impl Flower {
    pub fn new(id: Id, position: Vec2<f32>) -> Self {
        let stats = FlowerStats::new(1.0, 2.0, 0.0);
        Self {
            id,
            position,
            stats,
        }
    }
    pub fn new_random(id: Id, position: Vec2<f32>) -> Self {
        let stats = FlowerStats::new_random();
        Self {
            id,
            position,
            stats,
        }
    }
}
