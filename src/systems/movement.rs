use super::System;
use components::{Position, Velocity};
use recs::EntityId;
use game::GameState;
use utils::clamp;
use constants;

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Velocity);
        state.ecs().collect_with(&filter, &mut ids);
        for id in ids {
            let pos = state.ecs().get::<Position>(id).unwrap();
            let vel = state.ecs().get::<Velocity>(id).unwrap();
            let new_x = clamp(pos.x + vel.x, 0.0, constants::SCREEN_WIDTH as f64);
            let new_y = clamp(pos.y + vel.y, 0.0, constants::SCREEN_HEIGHT as f64);
            let _ = state.ecs().set(id, Position{x: new_x, y: new_y});
        }
    }
}