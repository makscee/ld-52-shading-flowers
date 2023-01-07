use super::*;

mod flower;

pub use flower::*;
pub struct Model {
    pub flowers: Collection<Flower>,
}

impl Model {
    pub fn new(logic: &mut Logic) -> Self {
        let mut flowers = Collection::new();
        flowers.insert(Flower::new(logic, Vec2::ZERO));
        Self { flowers }
    }
}
