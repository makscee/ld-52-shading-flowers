use super::*;

mod stats;

pub use stats::*;

#[derive(HasId, Clone)]
pub struct Flower {
    pub id: Id,
    pub position: Vec2<f32>,
    pub stats: FlowerStats,
    pub head: Option<Box<Flower>>,
    pub binds: HashMap<Id, Bind>,
}

impl Flower {
    pub fn new_random(id: Id, position: Vec2<f32>) -> Self {
        let stats = FlowerStats::new_random();
        Self {
            id,
            position,
            stats,
            head: None,
            binds: default(),
        }
    }
    pub fn new_stats(id: Id, position: Vec2<f32>, stats: FlowerStats) -> Self {
        Self {
            id,
            position,
            stats,
            head: None,
            binds: default(),
        }
    }
    pub fn new_offspring(id: Id, position: Vec2<f32>, parents: Vec<FlowerStats>) -> Self {
        Self::new_stats(id, position, FlowerStats::new_offspring(parents))
    }

    pub fn grow(&mut self) {
        if let Some(head) = &mut self.head {
            head.grow();
        } else {
            self.head = Some(Box::new(self.new_grown_head()));
        }
    }

    fn new_grown_head(&self) -> Flower {
        let mut new_head = self.clone();
        new_head.stats.radius *= 0.5;
        new_head.stats.size *= 0.5;
        new_head
    }

    pub fn is_mouse_over_size(&self, mouse_pos: Vec2<f32>) -> bool {
        (self.position - mouse_pos).len() < self.stats.size
    }

    pub fn is_mouse_over_radius(&self, mouse_pos: Vec2<f32>) -> bool {
        (self.position - mouse_pos).len() < self.stats.radius
    }

    pub fn handle_click(&self, harvest: &mut i32) {
        *harvest += 1;
    }

    pub fn update_binds(&mut self, delta_time: f32, model: &Model) {
        for bind in self.binds.values() {
            self.position += bind.get_delta_pos(delta_time, model);
        }
        self.binds.values_mut().for_each(|b| b.a = self.position);
        self.binds.retain(|_, v| !v.is_broken(model));
    }

    pub fn start_drag(&mut self) {
        self.add_bind(&0, Vec2::ZERO);
    }

    pub fn end_drag(&mut self) {
        self.remove_bind(&0);
    }

    pub fn add_bind(&mut self, id: &Id, offset: Vec2<f32>) {
        self.binds.insert(
            *id,
            Bind {
                a: self.position,
                b: *id,
                offset,
                tension: 15.0,
                toughness: 10.0,
            },
        );
    }

    pub fn remove_bind(&mut self, id: &Id) {
        if self.binds.contains_key(id) {
            self.binds.remove(id);
        }
    }
}
