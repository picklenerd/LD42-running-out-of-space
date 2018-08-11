use super::entity::{ Entity };

pub struct Game {
    next_id: u64,
    entities: Vec<Entity>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
