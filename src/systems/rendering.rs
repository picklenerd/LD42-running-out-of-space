use super::System;
use components::{Position, Renderer};
use recs::{Ecs, EntityId};

use pixi::Positionable;

pub struct RenderingSystem;

impl System for RenderingSystem {
    fn run(&mut self, ecs: &mut Ecs, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Renderer);
        ecs.collect_with(&filter, &mut ids);
        for id in ids {
            let pos = ecs.get::<Position>(id).unwrap();
            let graphics = ecs.get::<Renderer>(id).unwrap().graphics;
            graphics.set_position(pos.x, pos.y);
        }
    }
}