use std::f64::consts;

use super::System;
use components::{Velocity, KeyboardControls};
use keyboard::Keyboard;
use recs::EntityId;
use game::GameState;
use constants;

pub struct ControlSystem {
    keyboard: Keyboard,
}

impl ControlSystem {
    pub fn new() -> Self {
        let keyboard = Keyboard::new();
        keyboard.track_key("KeyW", "up");
        keyboard.track_key("KeyS", "down");
        keyboard.track_key("KeyA", "left");
        keyboard.track_key("KeyD", "right");

        Self { keyboard }
    }
}

impl System for ControlSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(KeyboardControls, Velocity);
        state.ecs().collect_with(&filter, &mut ids);
        for id in ids {
            let mut x_dir: f64 = 0.0;
            let mut y_dir: f64 = 0.0;

            if self.keyboard.key_down("left") {
                x_dir = -1.0;
            } else if self.keyboard.key_down("right") {
                x_dir = 1.0;
            }

            if self.keyboard.key_down("up") {
                y_dir = -1.0;
            } else if self.keyboard.key_down("down") {
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