use super::*;

pub struct FlowerStats {
    pub time_alive: Time,
    pub growth: f32,

    pub size: f32,
    pub radius: f32,
    pub hue: f32,
}

impl FlowerStats {
    pub fn new(size: f32, radius: f32, hue: f32) -> Self {
        Self {
            size,
            radius,
            hue,
            time_alive: 0.0,
            growth: 0.0,
        }
    }

    pub fn new_random() -> Self {
        Self {
            time_alive: 0.0,
            growth: 0.0,
            size: global_rng().gen_range(0.5..=1.0),
            radius: global_rng().gen_range(0.5..=1.0),
            hue: global_rng().gen_range(0.0..=1.0),
        }
    }

    pub fn update(&mut self, delta_time: Time) {
        self.time_alive += delta_time;
        self.growth = (self.time_alive * self.time_alive).min(1.0) as f32;
    }
}
