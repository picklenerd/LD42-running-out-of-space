#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;

pub mod pixi;
pub mod keyboard;
pub mod ecs;

use ::ecs::game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}