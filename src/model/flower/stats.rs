use super::*;

#[derive(Clone)]
pub struct FlowerStats {
    pub time_alive: Time,
    pub growth: f32,

    pub size: f32,
    pub radius: f32,
    pub color: Rgba<f32>,
    pub hue_shift: f32,
}

impl FlowerStats {
    pub fn new(size: f32, radius: f32, color: Rgba<f32>, hue_shift: f32) -> Self {
        Self {
            size,
            radius,
            color,
            time_alive: 0.0,
            growth: 0.0,
            hue_shift,
        }
    }

    pub fn new_random() -> Self {
        let color: Rgba<f32> = Rgba::new(
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            1.0,
        );
        Self {
            time_alive: 0.0,
            growth: 0.0,
            size: global_rng().gen_range(0.5..=1.0),
            radius: global_rng().gen_range(1.5..=2.5),
            color,
            hue_shift: global_rng().gen_range(0.0..0.1),
        }
    }

    pub fn new_offspring(parents: Vec<FlowerStats>) -> Self {
        let mut stats = FlowerStats::new(0.0, 0.0, Rgba::BLACK, 0.0);

        let sum: f32 = parents.iter().map(|f| f.radius).sum();
        stats.radius = sum / parents.len() as f32;

        stats.color = Self::get_color(&parents);

        let sum: f32 = parents.iter().map(|f| f.size).sum();
        stats.size = sum / parents.len() as f32;

        stats
    }

    fn get_color(parents: &Vec<FlowerStats>) -> Rgba<f32> {
        let mut color = Rgba::BLACK;
        for c in parents.iter().map(|p| p.color) {
            color.r = (c.r + color.r) * global_rng().gen_range(0.4..0.6);
            color.g = (c.g + color.g) * global_rng().gen_range(0.4..0.6);
            color.b = (c.b + color.b) * global_rng().gen_range(0.4..0.6);
        }
        color
    }

    pub fn update(&mut self, delta_time: Time) {
        self.time_alive += delta_time;
        self.growth = (self.time_alive * self.time_alive).min(1.0) as f32;
    }
}
