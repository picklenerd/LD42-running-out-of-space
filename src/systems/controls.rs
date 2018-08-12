use super::System;
use components::tags::{Projectile, Player, KeyboardControls};
use components::movement::{Velocity, Position};
use components::graphics::Renderer;
use components::colliders::Collider;
use input::Input;
use recs::EntityId;
use game::GameState;
use constants;
use pixi::graphics::Graphics;

pub struct ControlSystem {
    input: Input,
    wait_for_reset: bool,
}

impl ControlSystem {
    pub fn new() -> Self {
        let input = Input::new();
        
        // Keyboard controls
        input.track_key("KeyW", "up");
        input.track_key("KeyS", "down");
        input.track_key("KeyA", "left");
        input.track_key("KeyD", "right");

        // Mouse controls
        input.track_mouse("0", "shoot");

        Self { 
            input,
            wait_for_reset: false,
        }
    }

    fn handle_shooting(&mut self, state: &mut GameState) {
        if self.wait_for_reset && !self.input.is_control_active("shoot") {
            self.wait_for_reset = false;
        }
        
        if !self.wait_for_reset && self.input.is_control_active("shoot") {
            self.wait_for_reset = true;

            let mut ids: Vec<EntityId> = Vec::new();
            let filter = component_filter!(Player, Position);
            state.ecs().collect_with(&filter, &mut ids);
            let start_pos = state.ecs().get::<Position>(ids[0]).unwrap();

            let circle = Graphics::new();
            circle.begin_fill(constants::PROJECTILE_COLOR);
            circle.draw_ellipse(0.0, 0.0, constants::PROJECTILE_SIZE, constants::PROJECTILE_SIZE);
            state.pixi().add_child(&circle);

            let mouse_pos = self.input.mouse_position();

            let (x_vel, y_vel) = start_pos.vector_to(&mouse_pos);

            let projectile = state.ecs().create_entity();
            let _ = state.ecs().set(projectile, Projectile);
            let _ = state.ecs().set(projectile, start_pos.clone());
            let _ = state.ecs().set(projectile, Velocity{x: x_vel * constants::PROJECTILE_SPEED, y: y_vel * constants::PROJECTILE_SPEED});
            let _ = state.ecs().set(projectile, Renderer{graphics: circle});
            let _ = state.ecs().set(projectile, Collider{position: start_pos, radius: constants::PLAYER_SIZE});
        }
    }

    fn handle_movement(&mut self, state: &mut GameState) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(KeyboardControls, Velocity);
        state.ecs().collect_with(&filter, &mut ids);
        for id in ids {
            let mut x_dir: f64 = 0.0;
            let mut y_dir: f64 = 0.0;

            if self.input.is_control_active("left") {
                x_dir = -1.0;
            } else if self.input.is_control_active("right") {
                x_dir = 1.0;
            }

            if self.input.is_control_active("up") {
                y_dir = -1.0;
            } else if self.input.is_control_active("down") {
                y_dir = 1.0;
            }

            if x_dir == 0.0 && y_dir == 0.0 {
                let _ = state.ecs().set(id, Velocity{x: 0.0, y: 0.0});
                return;
            }

            let angle = y_dir.atan2(x_dir);

            let _ = state.ecs().set(id, Velocity{x: angle.cos() * constants::PLAYER_SPEED, y: angle.sin() * constants::PLAYER_SPEED});
        }
    }
}

impl System for ControlSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        self.handle_shooting(state);
        self.handle_movement(state);
    }
}