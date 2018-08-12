use recs::EntityId;

use constants;
use systems::System;
use game::GameState;
use components::movement::{Position, Velocity};
use components::graphics::Renderer;
use components::tags::{Player, KeyboardControls, Wall, Enemy, IceBlock};
use components::colliders::SquareCollider;
use pixi::graphics::Graphics;

pub struct PlayerSystem;

impl System for PlayerSystem {
    fn init(&mut self, state: &mut GameState) {
        let player_circle = Graphics::new();
        player_circle.begin_fill(constants::PLAYER_COLOR);
        player_circle.draw_ellipse(0.0, 0.0, constants::PLAYER_SIZE, constants::PLAYER_SIZE);
        state.app().add_child(&player_circle);

        let start_pos = Position{x: constants::PLAYER_START_X, y: constants::PLAYER_START_Y};

        let player = state.ecs().create_entity();
        let _ = state.ecs().set(player, Player);
        let _ = state.ecs().set(player, start_pos.clone());
        let _ = state.ecs().set(player, Velocity{x: 0.0, y: 0.0});
        let _ = state.ecs().set(player, Renderer{graphics: player_circle});
        let _ = state.ecs().set(player, KeyboardControls);
        let _ = state.ecs().set(player, SquareCollider{position: start_pos, width: constants::PLAYER_SIZE, height: constants::PLAYER_SIZE});
    }
    
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut player_ids = Vec::new();
        state.ecs().collect_with(&component_filter!(Player), &mut player_ids);
        let player = player_ids[0];
        
        let mut blocker_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Wall, SquareCollider), &mut blocker_ids);
        let mut ice_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(IceBlock, SquareCollider), &mut ice_ids);

        blocker_ids.append(&mut ice_ids);

        let player_coll = state.ecs().get::<SquareCollider>(player).unwrap();
        for blocker in &blocker_ids {
            let blocker_coll = state.ecs().get::<SquareCollider>(*blocker).unwrap();
            let (x_dir, y_dir) = blocker_coll.collision_direction(&player_coll);
            if (x_dir, y_dir) != (0.0, 0.0) {
                let _ = state.ecs().set(player, Velocity{x: -x_dir * constants::BOUNCE_SPEED, y: -y_dir * constants::BOUNCE_SPEED});
            }
        }
    }
}