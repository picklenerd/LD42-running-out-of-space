use super::System;
use components::movement::{Position, Velocity};
use components::colliders::SquareCollider;

use recs::EntityId;
use game::GameState;

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Position, Velocity), &mut ids);

        for id in ids {
            let pos = state.ecs().get::<Position>(id).unwrap();
            let vel = state.ecs().get::<Velocity>(id).unwrap();
            let new_pos = Position{x: pos.x + vel.x, y: pos.y + vel.y};
            let _ = state.ecs().set(id, new_pos.clone());

            if let Ok(mut coll) = state.ecs().borrow_mut::<SquareCollider>(id) {
                coll.set_pos(&new_pos);
            }
        }
    }
}