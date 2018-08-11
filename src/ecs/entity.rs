use super::{ Unique };

pub struct Entity {
    id: u64,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Unique for Entity {
    fn id(&self) -> u64 {
        self.id
    }
}