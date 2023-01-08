use std::borrow::Borrow;

use super::*;

mod stats;

pub use stats::*;

#[derive(HasId, Clone)]
pub struct Flower {
    pub id: Id,
    pub position: Vec2<f32>,
    pub stats: FlowerStats,
    pub head: Option<Id>,
    pub tail: Option<Id>,
    pub binds: HashMap<Id, Bind>,
    pub popped: bool,
    pub seed: bool,
}

impl Flower {
    pub fn new_random(id: Id, position: Vec2<f32>) -> Self {
        let stats = FlowerStats::new_random();
        let flower = Self {
            id,
            position,
            stats,
            head: None,
            tail: None,
            binds: default(),
            seed: false,
            popped: false,
        };
        flower
    }
    pub fn new_stats(id: Id, position: Vec2<f32>, stats: FlowerStats) -> Self {
        Self {
            id,
            position,
            stats,
            head: None,
            tail: None,
            binds: default(),
            seed: false,
            popped: false,
        }
    }
    pub fn new_offspring(id: Id, position: Vec2<f32>, parents: Vec<FlowerStats>) -> Self {
        Self::new_stats(id, position, FlowerStats::new_offspring(parents))
    }

    pub fn grow(&mut self, next_id: &Id, flowers: &mut Collection<Flower>) -> Flower {
        if let Some(head) = self.head {
            let mut head = flowers.remove(&head).expect("Flower not found");
            let new_head = head.grow(next_id, flowers);
            flowers.insert(head);
            new_head
        } else {
            let mut head = self.new_grown_head(next_id);
            head.bind_by_id(&self.id, vec2(0.0, -3.0));
            head.tail = Some(self.id);
            let head = head;
            debug!("new head#{}", head.id);
            self.head = Some(head.id);
            head
        }
    }

    pub fn get_all_nodes(&self, flowers: &Collection<Flower>) -> Vec<Id> {
        let mut nodes = vec![self.id];
        let mut node = self;
        while let Some(head) = node.head {
            nodes.push(head);
            node = flowers.get(&head).expect("Flower not found");
        }
        node = self;
        while let Some(tail) = node.tail {
            nodes.push(tail);
            node = flowers.get(&tail).expect("Flower not found");
        }
        nodes
    }

    pub fn get_root(&self, flowers: &Collection<Flower>) -> Id {
        let mut node = self;
        while let Some(tail) = node.tail {
            node = flowers.get(&tail).expect("Tail not found");
        }
        node.id
    }

    pub fn pop(&mut self) {
        self.stats.growth = 0.0;
        self.head = None;
        if self.tail.is_none() {
            return;
        }
        debug!("Popped#{}", self.id);
        self.popped = true;
    }

    fn new_grown_head(&self, next_id: &Id) -> Flower {
        let mut new_head = self.clone();
        new_head.id = *next_id;
        new_head.stats.radius *= 0.5;
        new_head.stats.size *= 0.5;
        new_head.seed = new_head.stats.size < 0.2;
        new_head.stats.growth = 0.0;
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

    pub fn update_binds(&mut self, delta_time: f32, model: &mut Model) {
        let mut node = self.head;
        self.do_update_binds(delta_time, model);
        while let Some(head) = &mut node {
            let mut head = model.flowers.remove(head).expect("Flower not found");
            head.do_update_binds(delta_time, model);
            node = head.head;
            model.flowers.insert(head);
        }
    }

    fn do_update_binds(&mut self, delta_time: f32, model: &Model) {
        for bind in self.binds.values() {
            self.position += bind.get_delta_pos(delta_time, model);
        }
        self.binds.values_mut().for_each(|b| b.a = self.position);
        self.binds.retain(|_, v| !v.is_broken(model));
    }

    pub fn has_ground_bind(&self) -> bool {
        self.binds.iter().any(|b| b.1.b < 0)
    }

    pub fn add_ground_bind(&mut self, id: Id) -> Bind {
        let bind = Bind {
            a: self.position,
            b: id,
            offset: Vec2::ZERO,
            tension: 10.0,
            toughness: 3.0,
        };
        self.bind_by_bind(bind.clone());
        bind
    }

    pub fn start_drag(&mut self) {
        self.bind_by_id(&0, Vec2::ZERO);
    }

    pub fn end_drag(&mut self) {
        self.remove_bind(&0)
    }

    pub fn bind_by_id(&mut self, id: &Id, offset: Vec2<f32>) {
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

    pub fn bind_by_bind(&mut self, bind: Bind) {
        self.binds.insert(bind.b, bind);
    }

    pub fn remove_bind(&mut self, id: &Id) {
        if self.binds.contains_key(id) {
            self.binds.remove(id);
        }
    }

    pub fn is_seed(&self, flowers: &Collection<Flower>) -> bool {
        if self.seed {
            return true;
        }
        let mut node = self;
        while let Some(head) = &node.head {
            let head = flowers.get(head).expect("Flower not found");
            if head.seed {
                return true;
            }
            node = &head;
        }
        return false;
    }

    fn is_grown(&self, flowers: &Collection<Flower>) -> bool {
        if self.stats.growth < 1.0 {
            return false;
        }
        let mut node = self;
        while let Some(head) = &node.head {
            let head = flowers.get(head).expect("Flower not found");
            if head.stats.growth < 1.0 {
                return false;
            }
            node = &head;
        }
        return true;
    }

    pub fn update_growth(
        &mut self,
        delta_time: f32,
        next_id: &mut Id,
        flowers: &mut Collection<Flower>,
    ) -> Option<Flower> {
        if !self.is_seed(flowers) && self.is_grown(flowers) {
            let id = *next_id;
            *next_id += 1;
            return Some(self.grow(&id, flowers));
        }
        None
    }
}
