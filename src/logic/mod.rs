use super::*;

mod bind;

pub use bind::*;
use geng::prelude::itertools::Itertools;

pub struct Logic {
    next_id: Id,
    pub model: Model,
}

impl Logic {
    pub fn new() -> Self {
        let next_id = 0;
        let model = Model::new();
        Self { next_id, model }
    }

    pub fn init(&mut self) {
        self.init_flowers();
    }

    fn init_flowers(&mut self) {
        let id = self.get_next_id();
        let mut flower = Flower::new_random(id, Vec2::ZERO);
        flower.add_bind(&0, Vec2::ZERO);
        self.model.flowers.insert(flower);
    }

    pub fn get_next_id(&mut self) -> Id {
        self.next_id += 1;
        self.next_id
    }

    pub fn update(&mut self, delta_time: f32) {
        let ids = self.model.flowers.ids().copied().collect_vec();
        for id in ids {
            let mut flower = self.model.flowers.remove(&id).expect("Unit not found");
            flower.update_binds(delta_time, &self.model);
            self.model.flowers.insert(flower);
        }
    }
}
