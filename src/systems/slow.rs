use recs::EntityId;

use systems::System;
use game::GameState;
use components::tags::IceBlock;
use components::movement::Slowable;
use components::colliders::Collider;

pub struct SlowSystem;

impl System for SlowSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Slowable, Collider), &mut ids);
        
        let mut ice_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(IceBlock, Collider), &mut ice_ids);
        for id in ids {
            let mut is_slowed = false;
            let coll = state.ecs().get::<Collider>(id).unwrap();
            for ice in &ice_ids {
                let ice_coll = state.ecs().get::<Collider>(*ice).unwrap();
                if coll.is_colliding(&ice_coll) {
                    is_slowed = true;
                    break;
                }
            }
            let slowable = state.ecs().borrow_mut::<Slowable>(id).unwrap();
            slowable.set(is_slowed);
        }
    }
}