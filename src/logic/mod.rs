use super::*;

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
        self.model.flowers.insert(Flower::new(id, Vec2::ZERO));
        let id = self.get_next_id();
        self.model
            .flowers
            .insert(Flower::new_random(id, vec2(2.0, 0.0)));
        let id = self.get_next_id();
        self.model
            .flowers
            .insert(Flower::new_random(id, vec2(-2.0, 0.0)));
    }

    pub fn get_next_id(&mut self) -> Id {
        self.next_id += 1;
        self.next_id
    }
}
