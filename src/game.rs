use std::f64::consts;

use stdweb::traits::*;
use stdweb::web::{
    document,
    window,
};
use rand::Rng;
use rand::rngs::OsRng;
use recs::{Ecs, EntityId};
use components::{Position, Velocity, Renderer, KeyboardControls};

use pixi::application::Application;
use pixi::graphics::Graphics;
use pixi::Positionable;
use keyboard::Keyboard;

pub struct Game {
    app: Application,
    keyboard: Keyboard,
    ecs: Ecs,
    entities: Vec<EntityId>,
    rng: OsRng,
}

impl Game {
    pub fn new() -> Self {
        let body = document().body().unwrap();
        let div = document().create_element("div").unwrap();
        
        body.append_child(&div);

        let app = Application::new(800, 600, 0xCCCCCC);
        body.append_child(&app.view());

        let player_circle = Graphics::new();
        player_circle.begin_fill(0x000000);
        player_circle.draw_ellipse(100, 100, 10, 10);
        app.add_child(&player_circle);

        let mut ecs = Ecs::new();

        let player = ecs.create_entity();
        let _ = ecs.set(player, Position{x: 200.0, y: 150.0});
        let _ = ecs.set(player, Velocity{x: 0.0, y: 0.0});
        let _ = ecs.set(player, Renderer{graphics: player_circle});
        let _ = ecs.set(player, KeyboardControls{});

        let mut entities = Vec::new();
        entities.push(player);

        let rng = OsRng::new().unwrap();

        let keyboard = Keyboard::new();
        keyboard.track_key("KeyW", "up");
        keyboard.track_key("KeyS", "down");
        keyboard.track_key("KeyA", "left");
        keyboard.track_key("KeyD", "right");

        Self {
            app,
            keyboard,
            ecs,
            entities,
            rng,
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.handle_keyboard();
        self.update_positions(delta);
        self.render();
    }

    fn handle_keyboard(&mut self) {
        let player_speed: f64 = 5.0;
        let deceleration: f64 = 2.0;
        

        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(KeyboardControls, Velocity);
        self.ecs.collect_with(&filter, &mut ids);
        for id in ids {
            let current = self.ecs.get::<Velocity>(id).unwrap();
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

            let _ = self.ecs.set(id, Velocity{x: new_x, y: new_y});
        }
    }

    fn update_positions(&mut self, delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Velocity);
        self.ecs.collect_with(&filter, &mut ids);
        for id in ids {
            let pos = self.ecs.get::<Position>(id).unwrap();
            let vel = self.ecs.get::<Velocity>(id).unwrap();
            let _ = self.ecs.set(id, Position{x: pos.x + vel.x, y: pos.y + vel.y});
        }
    }

    fn render(&mut self) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Renderer);
        self.ecs.collect_with(&filter, &mut ids);
        for id in ids {
            let pos = self.ecs.get::<Position>(id).unwrap();
            let graphics = self.ecs.get::<Renderer>(id).unwrap().graphics;
            graphics.set_x(pos.x);
            graphics.set_y(pos.y);
        }
    }
}
