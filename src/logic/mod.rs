use super::*;

pub struct Logic {
    next_id: Id,
}

impl Logic {
    pub fn new() -> Self {
        let next_id = 0;
        Self { next_id }
    }

    pub fn get_next_id(&mut self) -> Id {
        self.next_id += 1;
        self.next_id
    }
}
