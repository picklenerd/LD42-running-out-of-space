#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;

pub mod pixi;
pub mod keyboard;
pub mod ecs;

use stdweb::traits::*;
use stdweb::web::{
    document,
    window,
};

use pixi::application::Application;
use pixi::graphics::Graphics;
use pixi::Position;
use keyboard::Keyboard;

fn main() {
    let body = document().body().unwrap();
    let div = document().create_element("div").unwrap();
    
    body.append_child(&div);

    let app = Application::new(800, 600, 0x000000);
    body.append_child(&app.view());

    let circle = Graphics::new();
    circle.line_width(1);
    circle.line_color(0xFFFFFF);
    circle.begin_fill(0xFFFFFF);
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

        window().request_animation_frame(move |_| animate(circle, keyboard));
    }

    window().request_animation_frame(move |_| animate(circle, keyboard));
}