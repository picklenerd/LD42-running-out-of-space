use std::f64::consts;

use super::System;
use components::{Velocity, KeyboardControls};
use keyboard::Keyboard;
use recs::EntityId;
use game::GameState;

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
        let player_speed: f64 = 5.0;
        let deceleration: f64 = 2.0;

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(KeyboardControls, Velocity);
        state.ecs().collect_with(&filter, &mut ids);
        for id in ids {
            let current = state.ecs().get::<Velocity>(id).unwrap();
            let mut new_x = current.x;
            let mut new_y = current.y;

            let mut x_down = false;
            let mut y_down = false;
            
            if self.keyboard.key_down("left") {
                new_x = -player_speed;
                x_down = true;
            } else if self.keyboard.key_down("right") {
                new_x = player_speed;
                x_down = true;
            } else {
                if current.x != 0.0 { 
                    let diff = current.x.abs().min(deceleration.abs());
                    new_x = current.x - (current.x.signum() * diff);
                }
            }

            if self.keyboard.key_down("up") {
                new_y = -player_speed;
                y_down = true;
            } else if self.keyboard.key_down("down") {
                new_y = player_speed;
                y_down = true;
            } else {
                if current.y != 0.0 { 
                    let diff = current.y.abs().min(deceleration.abs());
                    new_y = current.y - (current.y.signum() * diff);
                }
            }

            if x_down && y_down {
                new_x /= consts::SQRT_2;
                new_y /= consts::SQRT_2;
            }

            let _ = state.ecs().set(id, Velocity{x: new_x, y: new_y});
        }
    }
}