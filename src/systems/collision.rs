use super::System;
use components::{Position, Collider};
use recs::EntityId;
use game::GameState;

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Collider);
        state.ecs().collect_with(&filter, &mut ids);
        for id in &ids {
            let pos = state.ecs().get::<Position>(*id).unwrap();
            let collider = state.ecs().get::<Collider>(*id).unwrap();
            for other in &ids {
                if id != other {
                    let other_pos = state.ecs().get::<Position>(*other).unwrap();
                    let other_collider = state.ecs().get::<Collider>(*other).unwrap();
                    if CollisionSystem::is_colliding(&pos, &collider, &other_pos, &other_collider) {
                        console!(log, "ouch!");
                    }
                }
            }
        }
    }
    
}

impl CollisionSystem {
    fn is_colliding(first_pos: &Position, first_coll: &Collider, second_pos: &Position, second_coll: &Collider) -> bool {
        (first_pos.x -second_pos.x).abs() <= ((first_coll.width as f64 / 2.0) + (second_coll.width as f64 / 2.0)) &&
        (first_pos.y - second_pos.y).abs() <= ((first_coll.height as f64 / 2.0) + (second_coll.height as f64 / 2.0))
    }
}