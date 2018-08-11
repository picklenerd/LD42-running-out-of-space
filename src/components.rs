use ::pixi::graphics::Graphics;

#[derive(Clone, PartialEq, Debug)]
pub struct Position{pub x: f64, pub y: f64}

#[derive(Clone, PartialEq, Debug)]
pub struct Velocity{pub x: f64, pub y: f64}

#[derive(Clone, PartialEq, Debug)]
pub struct Health{pub health: i32}

#[derive(Clone, PartialEq, Debug)]
pub struct Renderer{pub graphics: Graphics}

#[derive(Clone, PartialEq, Debug)]
pub struct KeyboardControls;

#[derive(Clone, PartialEq, Debug)]
pub struct Player;

#[derive(Clone, PartialEq, Debug)]
pub struct Enemy;

#[derive(Clone, PartialEq, Debug)]
pub struct Projectile;

#[derive(Clone, PartialEq, Debug)]
pub struct Collider{pub position: Position, pub width: u32, pub height: u32}

impl Collider {
    pub fn set_pos(&mut self, position: &Position) {
        self.position = position.clone();
    }

    pub fn is_colliding(&self, other: &Collider) -> bool {
        (self.position.x - other.position.x).abs() <= ((self.width as f64 / 2.0) + (other.width as f64 / 2.0)) &&
        (self.position.y - other.position.y).abs() <= ((self.height as f64 / 2.0) + (other.height as f64 / 2.0))
    }
}

impl Position {
    pub fn vector_to(&self, other: &Position) -> (f64, f64) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        let angle = y_diff.atan2(x_diff);

        (angle.cos(), angle.sin())
    }
}