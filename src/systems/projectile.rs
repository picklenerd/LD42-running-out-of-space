use recs::EntityId;

use pixi::sprite::Sprite;
use systems::System;
use game::GameState;
use components::colliders::Collider;
use components::movement::Position;
use components::graphics::Renderer;
use components::tags::{Projectile, IceBlock, Enemy};
use components::damage::{Health, Damage};
use constants;
use pixi::Sizable;

pub struct ProjectileSystem;

impl ProjectileSystem {
    fn splat(&self, state: &mut GameState, projectile: EntityId, turn_into_ice: bool) {
        let renderer = state.ecs().get::<Renderer>(projectile).unwrap();
        state.pixi().remove_child(&renderer.sprite);

        if turn_into_ice {
            let position = state.ecs().get::<Position>(projectile).unwrap();

            let sprite = Sprite::new("ice");
            sprite.set_square_size(constants::ICE_BLOCK_SIZE as f64);
            state.pixi().add_child_at(&sprite, 0);
            
            let ice = state.ecs().create_entity();
            let _ = state.ecs().set(ice, IceBlock);
            let _ = state.ecs().set(ice, Collider{position, radius: constants::ICE_BLOCK_SIZE});

        }

        let _ = state.ecs().destroy_entity(projectile);
    }
}

impl System for ProjectileSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut projectile_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Projectile), &mut projectile_ids);

        let mut blocker_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(IceBlock), &mut blocker_ids);
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Enemy), &mut enemy_ids);

        blocker_ids.append(&mut enemy_ids);

        for projectile in projectile_ids {
            let position = state.ecs().get::<Position>(projectile).unwrap();
            if position.x <= 0.0 || position.x >= constants::SCREEN_WIDTH as f64 || position.y <= 0.0 || position.y >= constants::SCREEN_HEIGHT as f64 {
                self.splat(state, projectile, true);
                continue;
            }

            let projectile_coll = state.ecs().get::<Collider>(projectile).unwrap();
            for blocker in &blocker_ids {
                if let Ok(bc) = state.ecs().get::<Collider>(*blocker) {
                    if bc.is_colliding(&projectile_coll) {
                            if let Ok(exists) = state.ecs().has::<Enemy>(*blocker) {
                            if exists {
                                if let Ok(damage) = state.ecs().get::<Damage>(projectile) {
                                    let health = state.ecs().borrow_mut::<Health>(*blocker).unwrap();
                                    health.amount -= damage.amount;
                                    js! { @(no_return)
                                        hitSound.play();
                                    }
                                }
                            }
                        }
                        self.splat(state, projectile, true);
                        break;
                    }
                }
            }
        }
    }
}
