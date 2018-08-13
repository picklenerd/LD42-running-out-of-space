#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;
extern crate recs;
extern crate rand;

#[macro_use]
pub mod utils;
pub mod constants;
pub mod pixi;
pub mod input;
pub mod game;
pub mod components;
pub mod systems;

use stdweb::traits::*;
use stdweb::web::{document, window, HtmlElement};
use stdweb::web::event::ClickEvent;
use stdweb::unstable::TryInto;

use ::game::Game;

fn main() {
    let loaded_callback = move || init();

    js! {@ (no_return)
        const callback = @{loaded_callback};
        PIXI.loader
            .add("penguin", "penguin.png")
            .add("bear", "bear.png")
            .add("shot", "shot.png")
            .add("ice", "ice.png")
            .load(() => {
                callback();
                callback.drop();
            });
    };
}

fn init() {
    let body = document().body().unwrap();
    
    let div = document().create_element("div").unwrap();
    div.set_attribute("id", "menu");
    body.append_child(&div);

    let title_text: HtmlElement = document().create_element("h1").unwrap().try_into().unwrap();
    title_text.set_text_content("Running Out of Space");
    div.append_child(&title_text);

    let start_button: HtmlElement = document().create_element("button").unwrap().try_into().unwrap();
    start_button.set_text_content("Start");
    div.append_child(&start_button);

    start_button.add_event_listener(move |_: ClickEvent| start_new_game());
}

fn start_new_game() {
    let menu = document().get_element_by_id("menu").unwrap();
    document().body().unwrap().remove_child(&menu);

    let mut game = Game::new();
    game.start();

    window().request_animation_frame(move |delta| update(game, delta));
}

fn update(mut game: Game, delta: f64) {
    game.update(delta);
    if game.state_ref().game_over() {
        set_game_over_screen(game.state_ref().score());
    } else {
        window().request_animation_frame(move |delta| update(game, delta));
    }
}

fn set_game_over_screen(score: i32) {
    let body = document().body().unwrap();
    if let Some(game_div) = document().get_element_by_id("game") {
        body.remove_child(&game_div).unwrap();
    }

    let div = document().create_element("div").unwrap();
    div.set_attribute("id", "menu");
    body.append_child(&div);

    let game_over_text: HtmlElement = document().create_element("h1").unwrap().try_into().unwrap();
    game_over_text.set_text_content("Game Over!");
    div.append_child(&game_over_text);

    let score_text: HtmlElement = document().create_element("h2").unwrap().try_into().unwrap();
    score_text.set_text_content(&format!("Score: {}", score));
    div.append_child(&score_text);

    let play_again_button: HtmlElement = document().create_element("button").unwrap().try_into().unwrap();
    play_again_button.set_text_content("Play Again");
    div.append_child(&play_again_button);

    play_again_button.add_event_listener(move |_: ClickEvent| start_new_game());
}