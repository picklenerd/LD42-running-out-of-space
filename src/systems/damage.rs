use recs::EntityId;

use systems::System;
use game::GameState;
use components::{Enemy, Player, Collider};

pub struct DamageSystem;

impl System for DamageSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Enemy, Collider), &mut enemy_ids);

        let mut player_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Player, Collider), &mut player_ids);

        let player_coll = state.ecs().get::<Collider>(player_ids[0]).unwrap();

        for id in enemy_ids {
            let coll = state.ecs().get::<Collider>(id).unwrap();
            if coll.is_colliding(&player_coll) {
                console!(log, "ouch!");
            }
        }
    }
}