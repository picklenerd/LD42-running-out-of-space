use super::System;
use components::tags::{Projectile, Player, KeyboardControls};
use components::movement::{Velocity, Position, Slowable};
use components::graphics::Renderer;
use components::colliders::Collider;
use components::damage::Damage;
use input::Input;
use recs::EntityId;
use game::GameState;
use constants;
use pixi::sprite::Sprite;
use pixi::{Sizable, Rotatable};

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
            let mut ids: Vec<EntityId> = Vec::new();
            let filter = component_filter!(Player, Position);
            state.ecs().collect_with(&filter, &mut ids);
            
            let slowable = state.ecs().get::<Slowable>(ids[0]).unwrap();
            if slowable.is_slowed {
               return; 
            }

            self.wait_for_reset = true;

            let start_pos = state.ecs().get::<Position>(ids[0]).unwrap();

            let sprite = Sprite::new("shot");
            sprite.set_square_size(constants::PROJECTILE_SIZE as f64);
            state.pixi().add_child(&sprite);

            let mouse_pos = self.input.mouse_position();

            let (x_vel, y_vel) = start_pos.vector_to(&mouse_pos);

            let projectile = state.ecs().create_entity();
            let _ = state.ecs().set(projectile, Projectile);
            let _ = state.ecs().set(projectile, start_pos.clone());
            let _ = state.ecs().set(projectile, Velocity{x: x_vel * constants::PROJECTILE_SPEED, y: y_vel * constants::PROJECTILE_SPEED});
            let _ = state.ecs().set(projectile, Renderer{sprite});
            let _ = state.ecs().set(projectile, Collider{position: start_pos, radius: constants::PLAYER_SIZE});
            let _ = state.ecs().set(projectile, Damage{amount: constants::PROJECTILE_DAMAGE});

            js! { @(no_return)
                shootSound.play();
            }
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

    fn handle_rotation(&mut self, state: &mut GameState) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Player);
        state.ecs().collect_with(&filter, &mut ids);
        let player = ids[0];

        let player_pos = state.ecs().get::<Position>(player).unwrap();
        let mouse_pos = self.input.mouse_position();

        let dx = mouse_pos.x - player_pos.x;
        let dy = mouse_pos.y - player_pos.y;
        
        let angle = dy.atan2(dx);

        let renderer = state.ecs().borrow_mut::<Renderer>(player).unwrap();
        renderer.sprite.set_angle(angle);
    }
}

impl System for ControlSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        self.handle_shooting(state);
        self.handle_movement(state);
        self.handle_rotation(state);
    }
}