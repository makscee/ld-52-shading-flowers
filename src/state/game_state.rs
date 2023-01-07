use super::*;

use geng::{prelude::itertools::Itertools, ui::*};

impl State for Game {
    fn update(&mut self, delta_time: f64) {
        self.logic.model.game_time += delta_time;
        self.logic.model.mouse_pos = self.view.camera.screen_to_world(
            self.view.framebuffer_size.map(|x| x as f32),
            self.geng.window().cursor_position().map(|x| x as f32),
        );
        debug!("Mouse pos: {}", self.logic.model.mouse_pos);
        
        self.logic.update(delta_time as f32);
        for flower in self.logic.model.flowers.iter_mut() {
            flower.stats.update(delta_time);
        }
    }
    fn fixed_update(&mut self, delta_time: f64) {}

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            Event::MouseDown { position, button } => {
                let position = self.logic.model.mouse_pos;
                for flower in self.logic.model.flowers.iter() {
                    if flower.is_mouse_over_size(position.map(|x| x as f32)) {
                        flower.handle_click(&mut self.logic.model.harvest);
                        return;
                    }
                }
                let intersections = self
                    .logic
                    .model
                    .flowers
                    .iter()
                    .filter(|f| f.is_mouse_over_radius(position))
                    .map(|f| f.stats.clone())
                    .collect_vec();
                let id = self.logic.get_next_id();
                if intersections.len() > 0 {
                    self.logic.model.flowers.insert(Flower::new_offspring(
                        id,
                        position,
                        intersections,
                    ));
                } else {
                    self.logic
                        .model
                        .flowers
                        .insert(Flower::new_random(id, position));
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

    fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        let text = self.logic.model.harvest.to_string();
        (
            ColorBox::new(Rgba::try_from("#21a91f").unwrap()).fixed_size(vec2(90.0, 90.0)),
            Text::new(text, cx.geng().default_font(), 32.0, Rgba::WHITE)
                .center()
                .fixed_size(vec2(35.0, 35.0)),
        )
            .stack()
            .align(vec2(0.05, 0.05))
            .boxed()
    }
}
