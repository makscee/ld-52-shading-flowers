mod assets;
mod game;
mod logic;
mod model;
mod state;
mod view;

use std::collections::*;

use assets::Assets;
use game::Game;
use geng::{prelude::*, *};
use logic::*;
use model::*;
use state::*;
use ugli::*;
use view::*;

type Id = i64;
type Name = String;
type Description = String;
type Time = f32;

fn setup_geng() -> Geng {
    geng::setup_panic_handler();
    let geng = Geng::new_with(geng::ContextOptions {
        title: "LD 52".to_owned(),
        shader_prefix: Some((
            include_str!("vertex_prefix.glsl").to_owned(),
            include_str!("fragment_prefix.glsl").to_owned(),
        )),
        target_ui_resolution: Some(vec2(1920.0, 1080.0)),
        ..default()
    });
    geng
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();
    let geng = setup_geng();

    let mut logic = Logic::new();
    let assets = Rc::new(
        futures::executor::block_on(<Assets as geng::LoadAsset>::load(&geng, &static_path()))
            .unwrap(),
    );

    let model = Rc::new(Model::new(&mut logic));
    let logic = Rc::new(logic);
    let view = Rc::new(View::new(geng.clone(), assets.clone(), model.clone()));

    let state = StateManager::new();
    let mut game = Game {
        geng: geng.clone(),
        logic: logic.clone(),
        assets: assets.clone(),
        view: view.clone(),
        state,
        model: model.clone(),
    };
    game.state.push(Box::new(GameState {
        model: model.clone(),
        view: view.clone(),
        logic: logic.clone(),
        assets: game.assets,
    }));
    geng::run(&geng, game.state);
}
