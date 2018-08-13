use super::System;
use components::movement::Position;
use components::graphics::Renderer;
use recs::EntityId;
use game::GameState;

use pixi::Positionable;

pub struct RenderingSystem;

impl System for RenderingSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Renderer);
        state.ecs().collect_with(&filter, &mut ids);
        for id in ids {
            let pos = state.ecs().get::<Position>(id).unwrap();
            let graphics = state.ecs().get::<Renderer>(id).unwrap().sprite;
            graphics.set_position(pos.x, pos.y);
        }
    }
}