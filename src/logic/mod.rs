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

    pub fn init(&mut self) {}

    pub fn get_next_id(&mut self) -> Id {
        self.next_id += 1;
        self.next_id
    }

    pub fn update(&mut self, delta_time: f32) {
        let ids = self.model.flowers.ids().copied().collect_vec();
        for id in ids {
            let mut flower = self.model.flowers.get(&id).expect("Unit not found").clone();
            flower.update_binds(delta_time, &self.model);
            let new_flower = flower.update_growth(delta_time, &mut self.next_id);
            if let Some(new_flower) = new_flower {
                self.model.flowers.insert(*new_flower);
            }
            self.model.flowers.remove(&flower.id);
            self.model.flowers.insert(flower);
        }
    }
}
