use super::System;
use ::components::{Position, Velocity};
use recs::{Ecs, EntityId};

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, ecs: &mut Ecs, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        let filter = component_filter!(Position, Velocity);
        ecs.collect_with(&filter, &mut ids);
        for id in ids {
            let pos = ecs.get::<Position>(id).unwrap();
            let vel = ecs.get::<Velocity>(id).unwrap();
            let _ = ecs.set(id, Position{x: pos.x + vel.x, y: pos.y + vel.y});
        }
    }
}