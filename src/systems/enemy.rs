use rand::Rng;
use rand::rngs::OsRng;

use recs::EntityId;

use constants;
use systems::System;
use components::{Position, Velocity, Renderer, Enemy};
use pixi::graphics::Graphics;
use game::GameState;

pub struct EnemySystem {
    enemy_count: u32,
    rng: OsRng,
}

impl EnemySystem {
    pub fn new() -> Self {
        Self {
            enemy_count: 0,
            rng: OsRng::new().unwrap(),
        }
    }

    fn spawn_enemy(&mut self, state: &mut GameState) {
        let enemy_circle = Graphics::new();
        enemy_circle.begin_fill(constants::ENEMY_COLOR);
        enemy_circle.draw_ellipse(0, 0, constants::PLAYER_SIZE, constants::PLAYER_SIZE);
        state.app().add_child(&enemy_circle);
        
        let enemy = state.ecs().create_entity();
        let start_pos = Position{
            x: self.rng.gen_range(0.0, constants::SCREEN_WIDTH as f64), 
            y: self.rng.gen_range(0.0, constants::SCREEN_HEIGHT as f64),
        };
        let _ = state.ecs().set(enemy, start_pos);
        let _ = state.ecs().set(enemy, Velocity{x: 0.0, y: 0.0});
        let _ = state.ecs().set(enemy, Enemy);
        let _ = state.ecs().set(enemy, Renderer{graphics: enemy_circle});
        self.enemy_count += 1;
    }
}

impl System for EnemySystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        if self.enemy_count == 0 {
            self.spawn_enemy(state);
        }
        
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Enemy);
        state.ecs().collect_with(&filter, &mut ids);
        for id in ids {
        }
    }
}