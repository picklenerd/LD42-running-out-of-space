pub mod game;
pub mod system;
pub mod component;
pub mod entity;

pub trait Unique {
    fn id(&self) -> u64;
}