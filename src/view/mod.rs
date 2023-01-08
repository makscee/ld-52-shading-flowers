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
        self.draw_flowers(framebuffer, model);
        self.draw_binds(framebuffer, model);
        self.draw_seed(framebuffer, model);
    }

    fn draw_shader<U>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        position: Vec2<f32>,
        shader: &ShaderProgram,
        uniforms: U,
    ) where
        U: Uniforms,
    {
        let mut instances_arr: ugli::VertexBuffer<Instance> =
            ugli::VertexBuffer::new_dynamic(self.geng.ugli(), Vec::new());
        instances_arr.resize(shader.instances, Instance {});
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
        )
    }

    fn draw_flowers(&self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        // let mut flowers = Vec::new();
        // for node in model.flowers.iter() {
        //     flowers.extend(node.get_all_nodes());
        // }
        for flower in model.flowers.iter() {
            let uniforms = uniforms!(
                u_radius: flower.stats.radius * flower.stats.growth,
                u_color: flower.stats.color,
                u_time: model.game_time,
            );
            self.draw_shader(
                framebuffer,
                flower.position,
                &self.assets.system_shaders.flower_radius,
                uniforms,
            );
        }
        for flower in model.flowers.iter() {
            let uniforms = uniforms!(
                u_size: flower.stats.size * flower.stats.growth,
                u_color: flower.stats.color,
                u_time: model.game_time,
            );
            self.draw_shader(
                framebuffer,
                flower.position,
                &self.assets.system_shaders.flower,
                uniforms,
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
                    u_color: flower.stats.color,
                    u_time: model.game_time,
                    u_position_2: Bind::get_position_by_id(bind.b, model),
                    u_toughness: bind.toughness,
                );
                self.draw_shader(
                    framebuffer,
                    bind.a,
                    &self.assets.system_shaders.bind,
                    uniforms,
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
        );
    }
}
