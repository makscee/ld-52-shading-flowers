use super::*;

mod shader;

use geng::prelude::itertools::Itertools;
pub use geng::Camera2d;
pub use shader::*;

pub struct View {
    pub camera: Camera2d,
    pub framebuffer_size: Vec2<usize>,
    geng: Geng,
    assets: Rc<Assets>,
}

impl View {
    pub fn new(geng: Geng, assets: Rc<Assets>) -> Self {
        let camera = geng::Camera2d {
            center: vec2(0.0, 0.0),
            rotation: 0.0,
            fov: 15.0,
        };
        Self {
            camera,
            geng,
            assets,
            framebuffer_size: vec2(100, 100),
        }
    }

    pub fn draw(&self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        self.draw_field(framebuffer, model);
        self.draw_binds(framebuffer, model);
        self.draw_flowers(framebuffer, model);
        self.draw_seed(framebuffer, model);
    }

    fn draw_shader<U>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<f32>,
        shader: &ShaderProgram,
        uniforms: U,
        instances: f32,
    ) where
        U: Uniforms,
    {
        let mut instances_arr: ugli::VertexBuffer<Instance> =
            ugli::VertexBuffer::new_dynamic(self.geng.ugli(), Vec::new());
        instances_arr.resize((shader.instances as f32 * instances) as usize, Instance {});
        let quad = shader.get_vertices(&self.geng);

        let program = shader.program.as_ref().expect("Shader program not loaded");
        ugli::draw(
            framebuffer,
            &program,
            ugli::DrawMode::TriangleStrip,
            ugli::instanced(&quad, &instances_arr),
            (
                ugli::uniforms! {
                    u_time: 0.0,
                    u_position: position
                },
                geng::camera2d_uniforms(&self.camera, self.framebuffer_size.map(|x| x as f32)),
                &shader.parameters,
                uniforms,
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::default()),
                ..default()
            },
        );
    }

    fn draw_field(&self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        self.draw_shader(
            framebuffer,
            Vec2::ZERO,
            &self.assets.system_shaders.field,
            uniforms!(),
            1.0,
        )
    }

    fn draw_flowers(&self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        for flower in model.flowers.iter() {
            let uniforms = uniforms!(
                u_radius: flower.stats.radius * flower.stats.growth,
                u_color_1: flower.stats.color_1,
                u_color_2: flower.stats.color_2,
                u_time: model.game_time,
            );
            self.draw_shader(
                framebuffer,
                flower.position,
                &self.assets.system_shaders.flower_radius,
                uniforms,
                1.0,
            );
        }
        for flower in model.flowers.iter() {
            let mut color = flower.stats.color_1;
            if flower.seed && flower.stats.growth >= 1.0 {
                let time = f32::sin(model.game_time as f32 * 0.3) * 0.5;
                color.r = time + color.r;
                let time = f32::sin(model.game_time as f32 * 0.7) * 0.5;
                color.g = time + color.g;
                let time = f32::sin(model.game_time as f32 * 1.3) * 0.5;
                color.b = time + color.b;
            }
            let uniforms = uniforms!(
                u_size: flower.stats.size * flower.stats.growth,
                u_color_1: color,
                u_color_2: flower.stats.color_2,
                u_time: model.game_time,
            );
            self.draw_shader(
                framebuffer,
                flower.position,
                &self.assets.system_shaders.flower,
                uniforms,
                1.0,
            );
        }
        for flower in model.flowers.iter() {
            let time = model.game_time as f32 + (0.6969429 * flower.id as f32) * 10.0;
            let mut color = flower.stats.color_1;
            if flower.seed && flower.stats.growth >= 1.0 {
                let time = f32::sin(model.game_time as f32 * 0.3) * 0.5;
                color.r = time + color.r;
                let time = f32::sin(model.game_time as f32 * 0.7) * 0.5;
                color.g = time + color.g;
                let time = f32::sin(model.game_time as f32 * 1.3) * 0.5;
                color.b = time + color.b;
            }
            let uniforms = uniforms!(
                u_color_1: color,
                u_color_2: flower.stats.color_2,
                u_time: time,
                u_mut_1: flower.stats.mutations[0],
                u_mut_2: flower.stats.mutations[1],
                u_mut_3: flower.stats.mutations[2],
                u_mut_4: flower.stats.mutations[3],
                u_mut_5: flower.stats.mutations[4],
            );
            self.draw_shader(
                framebuffer,
                flower.position,
                &self.assets.system_shaders.flower_particles,
                uniforms,
                flower.stats.radius
                    * flower.stats.growth
                    * flower.stats.growth
                    * (0.9 + flower.stats.mutations[3]),
            );
        }
    }

    fn draw_binds(&self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        for flower in model.flowers.iter() {
            for bind in flower.binds.values() {
                if bind.b == 0 {
                    continue;
                }
                let uniforms = uniforms!(
                    u_color_1: flower.stats.color_1,
                    u_color_2: flower.stats.color_2,
                    u_time: model.game_time,
                    u_position_2: Bind::get_position_by_id(bind.b, model),
                    u_toughness: bind.toughness,
                );
                self.draw_shader(
                    framebuffer,
                    bind.a,
                    &self.assets.system_shaders.bind,
                    uniforms,
                    1.0,
                );
            }
        }
    }

    fn draw_seed(&self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        if !model.seed {
            return;
        }
        let uniforms = uniforms!(
            u_time: model.game_time,
        );
        self.draw_shader(
            framebuffer,
            model.mouse_pos,
            &self.assets.system_shaders.seed,
            uniforms,
            1.0,
        );
    }
}
