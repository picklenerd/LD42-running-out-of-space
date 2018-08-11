#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;

pub mod pixi;
pub mod keyboard;
pub mod ecs;

use stdweb::web::window;

use ::ecs::game::Game;

fn main() {
    let game = Game::new();
    window().request_animation_frame(move |delta| update(game, delta));
}

fn update(mut game: Game, delta: f64) {
    game.update(delta);
    window().request_animation_frame(move |delta| update(game, delta));
}
