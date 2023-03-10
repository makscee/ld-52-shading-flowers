use super::*;

#[derive(Clone)]
pub struct Bind {
    pub a: Vec2<f32>,
    pub b: Id,
    pub offset: Vec2<f32>,
    pub tension: f32,
    pub toughness: f32, // max length before break
}

impl Bind {
    pub fn is_broken(&self, model: &Model) -> bool {
        let distance = (self.a - Self::get_position_by_id(self.b, model)).len();
        self.toughness > 0.0 && distance > self.toughness
    }

    pub fn get_position_by_id(id: Id, model: &Model) -> Vec2<f32> {
        if id == 0 {
            model.mouse_pos
        } else if id > 0 {
            model.flowers.get(&id).map_or(Vec2::ZERO, |f| f.position)
        } else {
            model.fixed_pos[&id]
        }
    }

    pub fn get_delta_pos(&self, delta_time: f32, model: &Model) -> Vec2<f32> {
        let a_pos = self.a;
        let b_pos = Self::get_position_by_id(self.b, model);
        let a_need_pos = b_pos - self.offset;
        let a_vel = (a_need_pos - a_pos) * self.tension * delta_time;
        a_vel
    }
}
