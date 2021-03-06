use stdweb::web::Date;

use rand::Rng;
use rand::rngs::OsRng;

use recs::EntityId;

use constants;
use systems::System;
use components::tags::{Player, Enemy};
use components::movement::{Position, Velocity, Slowable};
use components::colliders::Collider;
use components::graphics::Renderer;
use components::damage::Health;
use pixi::sprite::Sprite;
use game::GameState;
use pixi::{Sizable, Rotatable};

pub struct EnemySystem {
    rng: OsRng,
    last_spawn_time: f64,
    current_level: u32,
}

impl EnemySystem {
    pub fn new() -> Self {
        Self {
            rng: OsRng::new().unwrap(),
            last_spawn_time: 0.0,
            current_level: 1,
        }
    }

    fn spawn_enemy(&mut self, state: &mut GameState) {
        let sprite = Sprite::new("bear");
        sprite.set_square_size(constants::ENEMY_SIZE as f64);
        state.pixi().add_child(&sprite);
        
        let enemy = state.ecs().create_entity();
        let start_pos = self.get_spawn_position();
        let _ = state.ecs().set(enemy, start_pos.clone());
        let _ = state.ecs().set(enemy, Velocity{x: 0.0, y: 0.0});
        let _ = state.ecs().set(enemy, Enemy);
        let _ = state.ecs().set(enemy, Health{amount: constants::ENEMY_HEALTH});
        let _ = state.ecs().set(enemy, Renderer{sprite});
        let _ = state.ecs().set(enemy, Collider{position: start_pos, radius: constants::ENEMY_SIZE * 2});
        let _ = state.ecs().set(enemy, Slowable::new(constants::ENEMY_SLOWED_MULTIPLIER));

        self.current_level += 1;
    }

    fn get_spawn_position(&mut self) -> Position { 
        let x: i32 = self.rng.gen_range(0, 2) * constants::SCREEN_WIDTH as i32;
        let y: i32 = self.rng.gen_range(0, 2) * constants::SCREEN_HEIGHT as i32;
      
        Position{x: x as f64, y: y as f64}
    }
}

impl System for EnemySystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Velocity, Enemy);
        state.ecs().collect_with(&filter, &mut enemy_ids);

        let now = Date::now();
        if now - self.last_spawn_time >= constants::TIME_BETWEEN_SPAWNS
            && enemy_ids.len() < constants::MAX_ENEMIES as usize
            && enemy_ids.len() <= (self.current_level / 4) as usize {
            self.spawn_enemy(state);
            self.last_spawn_time = now;
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

        let renderer = state.ecs().borrow_mut::<Renderer>(enemy_id).unwrap();
        renderer.sprite.set_angle(y_dir.atan2(x_dir) - 1.57);
    }
}