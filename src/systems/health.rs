use recs::EntityId;

use systems::System;
use game::GameState;
use components::tags::{Enemy, Player};
use components::colliders::Collider;
use components::damage::Health;
use components::graphics::Renderer;

pub struct HealthSystem;

impl System for HealthSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Enemy, Collider), &mut enemy_ids);

        let mut player_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Player, Collider), &mut player_ids);

        let player_coll = state.ecs().get::<Collider>(player_ids[0]).unwrap();

        for id in enemy_ids {
            // Check to see if they player got eaten
            let coll = state.ecs().get::<Collider>(id).unwrap();
            if coll.is_colliding(&player_coll) {
                console!(log, "ouch!");
            }

            // Check to see if the enemy is dead
            if let Ok(health) = state.ecs().get::<Health>(id) {
                if health.amount <= 0 {
                    let renderer = state.ecs().get::<Renderer>(id).unwrap();
                    state.pixi().remove_child(&renderer.graphics);
                    let _ = state.ecs().destroy_entity(id);
                    continue;
                }
            }
        }

    }
}