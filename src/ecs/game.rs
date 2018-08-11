use stdweb::traits::*;
use stdweb::web::{
    document,
    window,
};

use pixi::application::Application;
use pixi::graphics::Graphics;
use pixi::Position;
use keyboard::Keyboard;

use super::entity::{ Entity };

pub struct Game {
    next_id: u64,
    app: Application,
    keyboard: Keyboard,
    player: Graphics,
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Self {
        let body = document().body().unwrap();
        let div = document().create_element("div").unwrap();
        
        body.append_child(&div);

        let app = Application::new(800, 600, 0xCCCCCC);
        body.append_child(&app.view());

        let circle = Graphics::new();
        circle.line_width(1);
        circle.line_color(0x000000);
        circle.begin_fill(0x000000);
        circle.draw_ellipse(100, 100, 10, 10);
        app.add_child(&circle);

        let keyboard = Keyboard::new();
        keyboard.track_key("KeyW", "up");
        keyboard.track_key("KeyS", "down");
        keyboard.track_key("KeyA", "left");
        keyboard.track_key("KeyD", "right");

        Self {
            next_id: 0,
            app,
            keyboard,
            player: circle,
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn update(&mut self, delta: f64) {
        self.update_positions(delta);
    }

    fn update_positions(&mut self, delta: f64) {
        if self.keyboard.key_down("up") {
            self.player.add_y(-2);
        } else if self.keyboard.key_down("down") {
            self.player.add_y(2);
        }

        if self.keyboard.key_down("left") {
            self.player.add_x(-2);
        } else if self.keyboard.key_down("right") {
            self.player.add_x(2);
        }
    }
}
