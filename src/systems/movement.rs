use super::System;
use game::GameState;
use components::movement::{Position, Velocity};
use components::tags::IceBlock;
use components::colliders::Collider;
use utils::clamp;
use constants;

use recs::EntityId;

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Position, Velocity), &mut ids);

        let mut ice_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(IceBlock, Collider), &mut ice_ids);

        for id in ids {
            let pos = state.ecs().get::<Position>(id).unwrap();
            let mut vel = state.ecs().get::<Velocity>(id).unwrap();

            if let Ok(mut coll) = state.ecs().get::<Collider>(id) {
                for ice in &ice_ids {
                    let ice_coll = state.ecs().get::<Collider>(*ice).unwrap();
                    if coll.is_colliding(&ice_coll) {
                        vel.x *= constants::ICE_SPEED_MULTIPLIER;
                        vel.y *= constants::ICE_SPEED_MULTIPLIER;
                        break;
                    }
                }
            }

            let new_pos = Position{
                x: clamp(pos.x + vel.x, 0.0, constants::SCREEN_WIDTH as f64),
                y: clamp(pos.y + vel.y, 0.0, constants::SCREEN_HEIGHT as f64),
            };
            let _ = state.ecs().set(id, new_pos.clone());

            if let Ok(mut coll) = state.ecs().borrow_mut::<Collider>(id) {
                coll.set_pos(&new_pos);
            }
        }
    }
}