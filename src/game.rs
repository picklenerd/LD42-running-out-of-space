use stdweb::traits::*;
use stdweb::web::{document, HtmlElement};
use stdweb::unstable::TryInto;

use constants;
use recs::Ecs;
use systems::System;
use systems::movement::MovementSystem;
use systems::rendering::RenderingSystem;
use systems::controls::ControlSystem;
use systems::enemy::EnemySystem;
use systems::player::PlayerSystem;
use systems::health::HealthSystem;
use systems::projectile::ProjectileSystem;
use systems::slow::SlowSystem;
use pixi::Pixi;

pub struct Game {
    state: GameState,
    systems: Vec<Box<System>>,
}

pub struct GameState {
    pixi: Pixi,
    ecs: Ecs,
    score: i32,
    game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        let body = document().body().unwrap();
        let div = document().create_element("div").unwrap();
        div.set_attribute("id", "game").unwrap();
        
        body.append_child(&div);

        let pixi = Pixi::new(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT, constants::BACKGROUND_COLOR);
        div.append_child(&pixi.view());

        Self { 
            pixi, 
            ecs: Ecs::new(),
            score: 0,
            game_over: false,
        }
    }

    pub fn pixi(&mut self) -> &mut Pixi {
        &mut self.pixi
    }

    pub fn ecs(&mut self) -> &mut Ecs {
        &mut self.ecs
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn trigger_game_over(&mut self) {
        self.game_over = true;
    }
}

impl Game {
    pub fn new() -> Self {
        let mut systems: Vec<Box<System>> = Vec::new();
        systems.push(Box::new(ControlSystem::new()));
        systems.push(Box::new(PlayerSystem));
        systems.push(Box::new(SlowSystem));
        systems.push(Box::new(MovementSystem));
        systems.push(Box::new(EnemySystem::new()));
        systems.push(Box::new(ProjectileSystem));
        systems.push(Box::new(HealthSystem));
        systems.push(Box::new(RenderingSystem));

        Self {
            state: GameState::new(),
            systems,
        }
    }

    pub fn init(&mut self) {
        for system in &mut self.systems {
            system.init(&mut self.state);
        }
    }

    pub fn update(&mut self, delta: f64) {
        for system in &mut self.systems {
            system.run(&mut self.state, delta);
        }
        if self.state.game_over {
            Game::set_game_over_screen(self.state.score);
        }
    }

    pub fn set_game_over_screen(score: i32) {
        let body = document().body().unwrap();
        if let Some(game_div) = document().get_element_by_id("game") {
            body.remove_child(&game_div).unwrap();
            
            let game_over_text: HtmlElement = document().create_element("h1").unwrap().try_into().unwrap();
            game_over_text.set_text_content("Game Over!");
            body.append_child(&game_over_text);

            let score_text: HtmlElement = document().create_element("h2").unwrap().try_into().unwrap();
            score_text.set_text_content(&format!("Score: {}", score));
            body.append_child(&score_text);
        }
    }
}
