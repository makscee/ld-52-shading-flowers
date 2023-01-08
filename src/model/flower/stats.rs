use super::*;

#[derive(Clone)]
pub struct FlowerStats {
    pub growth: f32,

    pub size: f32,
    pub radius: f32,
    pub color_1: Rgba<f32>,
    pub color_2: Rgba<f32>,
    pub hue_shift: f32,
    pub mutations: Vec<f32>,
}

impl FlowerStats {
    pub fn new(
        size: f32,
        radius: f32,
        color_1: Rgba<f32>,
        color_2: Rgba<f32>,
        hue_shift: f32,
    ) -> Self {
        Self {
            size,
            radius,
            color_1,
            color_2,
            growth: 0.0,
            hue_shift,
            mutations: default(),
        }
    }

    pub fn new_random() -> Self {
        let color_1: Rgba<f32> = Rgba::new(
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            1.0,
        );
        let color_2: Rgba<f32> = Rgba::new(
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            1.0,
        );
        let mutations = vec![
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
        ];
        Self {
            growth: 0.0,
            size: global_rng().gen_range(0.5..=1.0),
            radius: global_rng().gen_range(1.5..=2.5),
            color_1,
            color_2,
            mutations,
            hue_shift: global_rng().gen_range(0.0..0.1),
        }
    }

    pub fn new_offspring(parents: Vec<FlowerStats>) -> Self {
        let mut stats = FlowerStats::new(0.0, 0.0, Rgba::BLACK, Rgba::BLACK, 0.0);

        let sum: f32 = parents.iter().map(|f| f.radius).sum();
        stats.radius = sum / parents.len() as f32;

        (stats.color_1, stats.color_2) = Self::get_color(&parents);

        let sum: f32 = parents.iter().map(|f| f.size).sum();
        stats.size = sum / parents.len() as f32;

        stats.mutations = Self::mix_mutations(parents);

        stats
    }

    fn mix_mutations(parents: Vec<FlowerStats>) -> Vec<f32> {
        let mut m = vec![];
        for i in 0..5 {
            let mut v: f32 = parents.iter().map(|p| p.mutations[i]).sum();
            v /= parents.len() as f32;
            m.push(v);
        }
        m
    }

    fn get_color(parents: &Vec<FlowerStats>) -> (Rgba<f32>, Rgba<f32>) {
        let mut color_1 = Rgba::new(
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            1.0,
        );
        let mut color_2 = Rgba::new(
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            global_rng().gen_range(0.0..=1.0),
            1.0,
        );
        for c in parents.iter().map(|p| p.color_1) {
            color_1.r = (c.r + color_1.r) * 0.5;
            color_1.g = (c.g + color_1.g) * 0.5;
            color_1.b = (c.b + color_1.b) * 0.5;
        }
        for c in parents.iter().map(|p| p.color_2) {
            color_2.r = (c.r + color_2.r) * 0.5;
            color_2.g = (c.g + color_2.g) * 0.5;
            color_2.b = (c.b + color_2.b) * 0.5;
        }
        debug!(
            "{} {} {}",
            color_1.to_string(),
            color_2.to_string(),
            parents.len()
        );
        (color_1, color_2)
    }

    pub fn update(&mut self, delta_time: Time) {
        self.growth += delta_time as f32 * 0.2 * (0.5 + 2.0 * self.mutations[0]);
        self.growth = self.growth.min(1.0);
    }
}
