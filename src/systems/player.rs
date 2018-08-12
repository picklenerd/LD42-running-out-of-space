use constants;
use systems::System;
use game::GameState;
use components::movement::{Position, Velocity, Slowable};
use components::graphics::Renderer;
use components::tags::{Player, KeyboardControls};
use components::colliders::Collider;
use pixi::graphics::Graphics;

pub struct PlayerSystem;

impl System for PlayerSystem {
    fn init(&mut self, state: &mut GameState) {
        let player_circle = Graphics::new();
        player_circle.begin_fill(constants::PLAYER_COLOR);
        player_circle.draw_ellipse(0.0, 0.0, constants::PLAYER_SIZE, constants::PLAYER_SIZE);
        state.pixi().add_child(&player_circle);

        let start_pos = Position{x: constants::PLAYER_START_X, y: constants::PLAYER_START_Y};

        let player = state.ecs().create_entity();
        let _ = state.ecs().set(player, Player);
        let _ = state.ecs().set(player, start_pos.clone());
        let _ = state.ecs().set(player, Velocity{x: 0.0, y: 0.0});
        let _ = state.ecs().set(player, Renderer{graphics: player_circle});
        let _ = state.ecs().set(player, KeyboardControls);
        let _ = state.ecs().set(player, Collider{position: start_pos, radius: constants::PLAYER_SIZE});
        let _ = state.ecs().set(player, Slowable::new(constants::PLAYER_SLOWED_MULTIPLIER));
    }
    
    fn run(&mut self, _state: &mut GameState, _delta: f64) {
    }
}