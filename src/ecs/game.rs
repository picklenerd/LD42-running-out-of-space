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
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn run(&mut self) {
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

        fn animate(circle: Graphics, keyboard: Keyboard) {
            if keyboard.key_down("up") {
                circle.add_y(-2);
            } else if keyboard.key_down("down") {
                circle.add_y(2);
            }

            if keyboard.key_down("left") {
                circle.add_x(-2);
            } else if keyboard.key_down("right") {
                circle.add_x(2);
            }

            window().request_animation_frame(|_| animate(circle, keyboard));
        }

        window().request_animation_frame(|_| animate(circle, keyboard));
    }

    fn update_positions(&mut self) {

    }
}
