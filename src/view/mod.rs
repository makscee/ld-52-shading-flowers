use super::*;

mod shader;

pub use geng::Camera2d;
pub use shader::*;

pub struct View {
    pub camera: Camera2d,
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
        }
    }

    pub fn draw(&self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        self.draw_field(framebuffer, model);
        self.draw_flowers(framebuffer, model);
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
        let framebuffer_size = framebuffer.size();

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
                geng::camera2d_uniforms(&self.camera, framebuffer_size.map(|x| x as f32)),
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
        for flower in model.flowers.iter() {
            let uniforms = uniforms!(
                u_radius: flower.stats.radius * flower.stats.growth,
                u_hue: flower.stats.hue,
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
                u_hue: flower.stats.hue,
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
}
