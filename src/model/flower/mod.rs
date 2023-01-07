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
    pub fn new_stats(id: Id, position: Vec2<f32>, stats: FlowerStats) -> Self {
        Self {
            id,
            position,
            stats,
        }
    }
    pub fn new_offspring(id: Id, position: Vec2<f32>, parents: Vec<FlowerStats>) -> Self {
        Self::new_stats(id, position, FlowerStats::new_offspring(parents))
    }

    pub fn is_mouse_over_size(&self, mouse_pos: Vec2<f32>) -> bool {
        (self.position - mouse_pos).len() < self.stats.size
    }

    pub fn is_mouse_over_radius(&self, mouse_pos: Vec2<f32>) -> bool {
        (self.position - mouse_pos).len() < self.stats.radius
    }

    pub fn handle_click(&self, harvest: &mut i32) {
        debug! {"click {}", self.id};
        *harvest += 1;
    }
}
