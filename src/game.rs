use stdweb::traits::*;
use stdweb::web::document;

use constants;
use recs::Ecs;
use systems::System;
use systems::movement::MovementSystem;
use systems::rendering::RenderingSystem;
use systems::controls::ControlSystem;
use systems::enemy::EnemySystem;
use systems::player::PlayerSystem;
use systems::collision::CollisionSystem;

use pixi::application::Application;

pub struct Game {
    state: GameState,
    systems: Vec<Box<System>>,
}

pub struct GameState {
    app: Application,
    ecs: Ecs,
}

impl GameState {
    pub fn new() -> Self {
        let body = document().body().unwrap();
        let div = document().create_element("div").unwrap();
        
        body.append_child(&div);

        let app = Application::new(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT, constants::BACKGROUND_COLOR);
        body.append_child(&app.view());

        Self {
            app,
            ecs: Ecs::new(),
        }
    }

    pub fn app(&mut self) -> &mut Application {
        &mut self.app
    }

    pub fn ecs(&mut self) -> &mut Ecs {
        &mut self.ecs
    }
}

impl Game {
    pub fn new() -> Self {
        let mut systems: Vec<Box<System>> = Vec::new();
        systems.push(Box::new(MovementSystem));
        systems.push(Box::new(RenderingSystem));
        systems.push(Box::new(ControlSystem::new()));
        systems.push(Box::new(PlayerSystem));
        systems.push(Box::new(EnemySystem::new()));
        systems.push(Box::new(CollisionSystem));

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
    }
}
