use super::*;

impl State for Game {
    fn update(&mut self, delta_time: f64) {
        self.logic.model.game_time += delta_time;
        for flower in self.logic.model.flowers.iter_mut() {
            flower.stats.update(delta_time);
        }
    }
    fn fixed_update(&mut self, delta_time: f64) {}

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            Event::MouseDown { position, button } => {
                let position = position.map(|x| x as f32);
                let position = self
                    .view
                    .camera
                    .screen_to_world(self.view.framebuffer_size.map(|x| x as f32), position);
                for flower in self.logic.model.flowers.iter() {
                    if flower.is_clicked(position.map(|x| x as f32)) {
                        flower.handle_click();
                    }
                }
            }
            _ => {}
        }
    }

    fn transition(&mut self) -> Option<geng::Transition> {
        None
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        clear(framebuffer, Some(Rgba::BLUE), None, None);
        self.view.framebuffer_size = framebuffer.size();
        self.view.draw(framebuffer, &self.logic.model);
    }
}
