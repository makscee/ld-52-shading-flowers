use std::os::macos::raw::stat;

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
    pub fn new(logic: &mut Logic, position: Vec2<f32>) -> Self {
        let id = logic.get_next_id();
        let stats = FlowerStats::new(1.0, 3.0);
        Self {
            id,
            position,
            stats,
        }
    }
}
