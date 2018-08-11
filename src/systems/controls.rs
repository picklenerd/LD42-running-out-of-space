use super::System;
use components::{Velocity, KeyboardControls};
use input::Input;
use recs::EntityId;
use game::GameState;
use constants;

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
}

impl System for ControlSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        if self.wait_for_reset && !self.input.is_control_active("shoot") {
            self.wait_for_reset = false;
        }
        
        if !self.wait_for_reset && self.input.is_control_active("shoot") {
            self.wait_for_reset = true;
            console!(log, self.input.mouse_position().0);
            console!(log, self.input.mouse_position().1);
        }

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