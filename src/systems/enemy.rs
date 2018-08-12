use rand::Rng;
use rand::rngs::OsRng;

use recs::EntityId;

use constants;
use systems::System;
use components::tags::{Player, Enemy};
use components::movement::{Position, Velocity};
use components::colliders::Collider;
use components::graphics::Renderer;
use pixi::graphics::Graphics;
use game::GameState;

pub struct EnemySystem {
    rng: OsRng,
}

impl EnemySystem {
    pub fn new() -> Self {
        Self {
            rng: OsRng::new().unwrap(),
        }
    }

    fn spawn_enemy(&mut self, state: &mut GameState) {
        let enemy_circle = Graphics::new();
        enemy_circle.begin_fill(constants::ENEMY_COLOR);
        enemy_circle.draw_ellipse(0.0, 0.0, constants::ENEMY_SIZE, constants::ENEMY_SIZE);
        state.pixi().add_child(&enemy_circle);
        
        let enemy = state.ecs().create_entity();
        let start_pos = self.get_spawn_position(state);
        let _ = state.ecs().set(enemy, start_pos.clone());
        let _ = state.ecs().set(enemy, Velocity{x: 0.0, y: 0.0});
        let _ = state.ecs().set(enemy, Enemy);
        let _ = state.ecs().set(enemy, Renderer{graphics: enemy_circle});
        let _ = state.ecs().set(enemy, Collider{position: start_pos, radius: constants::ENEMY_SIZE / 2});
    }

    fn get_spawn_position(&mut self, state: &mut GameState) -> Position { 
        let mut player_ids = Vec::new();
        state.ecs().collect_with(&component_filter!(Player), &mut player_ids);
        let pos = state.ecs().get::<Position>(player_ids[0]).unwrap();

        let mut x = pos.x;
        let mut y = pos.y;

        while (x - pos.x).abs() < constants::ENEMY_MIN_SPAWN_DISTANCE {
            x = self.rng.gen_range(0.0, constants::SCREEN_WIDTH as f64);
        }
        while (y - pos.y).abs() < constants::ENEMY_MIN_SPAWN_DISTANCE {
            y = self.rng.gen_range(0.0, constants::SCREEN_HEIGHT as f64);
        }
        
        Position{x, y}
    }
}

impl System for EnemySystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Velocity, Enemy);
        state.ecs().collect_with(&filter, &mut enemy_ids);

        if enemy_ids.len() == 0 {
            self.spawn_enemy(state);
        }

        let mut player_ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Player);
        state.ecs().collect_with(&filter, &mut player_ids);

        let player = player_ids[0];
        let player_position = state.ecs().get::<Position>(player).unwrap();

        for enemy_id in enemy_ids {
            EnemySystem::chase_player(state, enemy_id, &player_position);
        }
    }
}

impl EnemySystem {
    pub fn chase_player(state: &mut GameState, enemy_id: EntityId, player_position: &Position) {
        let pos = state.ecs().get::<Position>(enemy_id).unwrap();

        let (x_dir, y_dir) = pos.vector_to(&player_position);

        let x_vel = constants::ENEMY_SPEED * x_dir;
        let y_vel = constants::ENEMY_SPEED * y_dir;

        let _ = state.ecs().set(enemy_id, Velocity{x: x_vel, y: y_vel});
    }
}