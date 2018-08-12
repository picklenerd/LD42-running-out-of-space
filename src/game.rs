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
use systems::damage::DamageSystem;
use systems::projectile::ProjectileSystem;
use pixi::application::Application;
use components::colliders::SquareCollider;
use components::movement::Position;
use components::tags::Wall;

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

        let mut ecs = Ecs::new();

        let top_wall = ecs.create_entity();
        let top_pos = Position{x: constants::SCREEN_WIDTH as f64 / 2.0, y: 0.0};
        let _ = ecs.set(top_wall, SquareCollider{position: top_pos, width: constants::SCREEN_WIDTH, height: 1});
        let _ = ecs.set(top_wall, Wall);
        
        let bottom_wall = ecs.create_entity();
        let bottom_pos = Position{x: constants::SCREEN_WIDTH as f64 / 2.0, y: constants::SCREEN_HEIGHT as f64};
        let _ = ecs.set(bottom_wall, SquareCollider{position: bottom_pos, width: constants::SCREEN_WIDTH, height: 1});
        let _ = ecs.set(bottom_wall, Wall);

        let left_wall = ecs.create_entity();
        let left_pos = Position{x: 0.0, y: constants::SCREEN_HEIGHT as f64 / 2.0};
        let _ = ecs.set(left_wall, SquareCollider{position: left_pos, width: 1, height: constants::SCREEN_HEIGHT});
        let _ = ecs.set(left_wall, Wall);

        let right_wall = ecs.create_entity();
        let right_pos = Position{x: constants::SCREEN_WIDTH as f64, y: constants::SCREEN_HEIGHT as f64 / 2.0};
        let _ = ecs.set(right_wall, SquareCollider{position: right_pos, width: 1, height: constants::SCREEN_HEIGHT});
        let _ = ecs.set(right_wall, Wall);

        Self { app, ecs }
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
        systems.push(Box::new(DamageSystem));
        systems.push(Box::new(ProjectileSystem));

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
