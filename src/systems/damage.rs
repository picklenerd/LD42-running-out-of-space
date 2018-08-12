use recs::EntityId;

use systems::System;
use game::GameState;
use components::tags::{Enemy, Player};
use components::colliders::SquareCollider;

pub struct DamageSystem;

impl System for DamageSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Enemy, SquareCollider), &mut enemy_ids);

        let mut player_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Player, SquareCollider), &mut player_ids);

        let player_coll = state.ecs().get::<SquareCollider>(player_ids[0]).unwrap();

        for id in enemy_ids {
            let coll = state.ecs().get::<SquareCollider>(id).unwrap();
            if coll.is_colliding(&player_coll) {
                console!(log, "ouch!");
            }
        }
    }
}