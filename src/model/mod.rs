use super::*;

mod flower;

pub use flower::*;

pub struct Model {
    pub game_time: Time,
    pub flowers: Collection<Flower>,
    pub harvest: i32,
}

impl Model {
    pub fn new() -> Self {
        let flowers = Collection::new();
        Self {
            flowers,
            game_time: 0.0,
            harvest: 0,
        }
    }
}
