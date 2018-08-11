pub mod movement;
pub mod rendering;
pub mod controls;

use recs::Ecs;

pub trait System {
    fn run(&mut self, ecs: &mut Ecs, delta: f64);
}