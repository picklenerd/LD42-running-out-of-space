use recs::EntityId;

use pixi::graphics::Graphics;
use systems::System;
use game::GameState;
use components::{Collider, Position, Renderer, Wall, Projectile, IceBlock, Enemy};
use constants;

pub struct ProjectileSystem;

impl System for ProjectileSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut projectile_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Projectile), &mut projectile_ids);

        let mut ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Wall), &mut ids);
        let mut ice_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(IceBlock), &mut ice_ids);
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Enemy), &mut enemy_ids);

        ids.append(&mut ice_ids);
        ids.append(&mut enemy_ids);

        for projectile in projectile_ids {
            let pc = state.ecs().get::<Collider>(projectile).unwrap();
            for id in &ids {
                if let Ok(wc) = state.ecs().get::<Collider>(*id) {
                    if wc.is_colliding(&pc) {
                        let renderer = state.ecs().get::<Renderer>(projectile).unwrap();
                        state.app().remove_child(&renderer.graphics);

                        let position = state.ecs().get::<Position>(projectile).unwrap();
                        let circle = Graphics::new();
                        circle.begin_fill(constants::ICE_BLOCK_COLOR);
                        circle.draw_ellipse(position.x as f64, position.y as f64, constants::ICE_BLOCK_SIZE, constants::ICE_BLOCK_SIZE);
                        state.app().add_child(&circle);
                        
                        let ice = state.ecs().create_entity();
                        let _ = state.ecs().set(ice, IceBlock);
                        let _ = state.ecs().set(ice, Collider{position, width: constants::ICE_BLOCK_SIZE, height: constants::ICE_BLOCK_SIZE});
                        
                        if let Ok(exists) = state.ecs().has::<Enemy>(*id) {
                            if exists {
                                let renderer = state.ecs().get::<Renderer>(*id).unwrap();
                                state.app().remove_child(&renderer.graphics);
                                let _ = state.ecs().destroy_entity(*id);
                            }
                        }

                        let _ = state.ecs().destroy_entity(projectile);
                        break;
                    }
                }
            }
        }
    }
}
