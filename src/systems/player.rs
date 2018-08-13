use constants;
use systems::System;
use game::GameState;
use components::movement::{Position, Velocity, Slowable};
use components::graphics::Renderer;
use components::tags::{Player, KeyboardControls};
use components::colliders::Collider;
use pixi::sprite::Sprite;
use pixi::Sizable;

pub struct PlayerSystem;

impl System for PlayerSystem {
    fn init(&mut self, state: &mut GameState) {
        let sprite = Sprite::new("penguin");
        sprite.set_square_size(constants::PLAYER_SIZE as f64);
        state.pixi().add_child(&sprite);

        let start_pos = Position{x: constants::PLAYER_START_X, y: constants::PLAYER_START_Y};

        let player = state.ecs().create_entity();
        let _ = state.ecs().set(player, Player);
        let _ = state.ecs().set(player, start_pos.clone());
        let _ = state.ecs().set(player, Velocity{x: 0.0, y: 0.0});
        let _ = state.ecs().set(player, Renderer{sprite});
        let _ = state.ecs().set(player, KeyboardControls);
        let _ = state.ecs().set(player, Collider{position: start_pos, radius: constants::PLAYER_SIZE});
        let _ = state.ecs().set(player, Slowable::new(constants::PLAYER_SLOWED_MULTIPLIER));
    }
    
    fn run(&mut self, _state: &mut GameState, _delta: f64) {
    }
}