use super::*;

use geng::{prelude::itertools::Itertools, ui::*};

impl State for Game {
    fn update(&mut self, delta_time: f64) {
        self.logic.model.game_time += delta_time;
        self.logic.model.mouse_pos = self.view.camera.screen_to_world(
            self.view.framebuffer_size.map(|x| x as f32),
            self.geng.window().cursor_position().map(|x| x as f32),
        );

        self.logic.update(delta_time as f32);
        for flower in self.logic.model.flowers.iter_mut() {
            flower.stats.update(delta_time);
        }
    }
    fn fixed_update(&mut self, delta_time: f64) {}

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            Event::MouseDown {
                position: _,
                button,
            } => {
                let position = self.logic.model.mouse_pos;
                let mut hovered_flower = None;
                let next_id = self.logic.get_next_id();
                for flower in self.logic.model.flowers.iter_mut() {
                    if flower.is_mouse_over_size(position.map(|x| x as f32)) {
                        hovered_flower = Some(flower);
                    }
                }
                if let Some(flower) = hovered_flower {
                    if button == MouseButton::Left {
                        flower.start_drag();
                    } else {
                        // let node = flower.grow(&next_id);
                        // debug!("new id#{}", node.id);
                        // self.logic.model.flowers.insert(*node);
                    }
                    return;
                }
                if !self.logic.model.seed {
                    return;
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
                    let mut new_flower = Flower::new_random(id, position);
                    let id = -self.logic.get_next_id();
                    let bind = new_flower.add_ground_bind(id);
                    self.logic.model.fixed_pos.insert(bind.b, bind.a);
                    self.logic.model.flowers.insert(new_flower);
                }
                self.logic.model.seed = false;
            }
            Event::MouseUp {
                position: _,
                button,
            } => {
                if button == MouseButton::Left {
                    let id = -self.logic.get_next_id();
                    for flower in self.logic.model.flowers.iter_mut() {
                        flower.end_drag();
                        if !flower.has_ground_bind() {
                            let bind = flower.add_ground_bind(id);
                            self.logic.model.fixed_pos.insert(bind.b, bind.a);
                        }
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
