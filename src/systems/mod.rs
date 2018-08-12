pub mod movement;
pub mod rendering;
pub mod controls;
pub mod player;
pub mod enemy;
pub mod damage;
pub mod projectile;
pub mod slow;

use game::GameState;

pub trait System {
    fn init(&mut self, state: &mut GameState) {}
    fn run(&mut self, state: &mut GameState, delta: f64);
}