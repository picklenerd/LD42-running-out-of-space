#![recursion_limit="128"]
#![feature(extern_prelude)]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate recs;
extern crate rand;

pub mod pixi;
pub mod keyboard;
pub mod game;
pub mod components;

use stdweb::web::window;

use ::game::Game;

fn main() {
    let game = Game::new();
    window().request_animation_frame(move |delta| update(game, delta));
}

fn update(mut game: Game, delta: f64) {
    game.update(delta);
    window().request_animation_frame(move |delta| update(game, delta));
}
