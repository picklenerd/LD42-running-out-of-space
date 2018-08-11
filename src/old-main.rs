#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;

pub mod pixi;

use stdweb::traits::*;
use stdweb::web::{
    document,
    window,
};

use pixi::application::Application;
use pixi::graphics::{ Graphics };
use pixi::sprite::{ Sprite };
use pixi::loader::{ Loader };
use pixi::{ Position };

fn main() {
    let body = document().body().unwrap();
    let div = document().create_element("div").unwrap();
    
    body.append_child(&div);

    let app = Application::new(800, 600, 0x000000);
    body.append_child(&app.view());
    
    let mut loader = Loader::new();
    loader.add("gorp", "gorp.jpg");

    loader.load(move || {
        let sprite = Sprite::new("gorp");
        app.add_child(&sprite);
        window().request_animation_frame(move |_| animate(&sprite));
    });
}

fn animate(object: &Sprite) {
    let x = object.get_x();
    object.set_x(x + 1);
    let other = object.clone();
    window().request_animation_frame(move |_| animate(&other));
}

// fn old() {
//     let rect = Graphics::new();
//     rect.line_width(1);
//     rect.line_color(0xFFFFFF);
//     rect.begin_fill(0xFFFFFF);
//     rect.draw_rect(100, 100, 100, 100);
//     app.add_child(&rect);

//     fn animate(rect: Graphics) {
//         let x = rect.get_x();
//         rect.set_x(x + 1);
//         window().request_animation_frame(move |_| animate(rect));
//     }

//     window().request_animation_frame(move |_| animate(rect));
// }