use components::movement::Position;

#[derive(Clone, PartialEq, Debug)]
pub struct SquareCollider{pub position: Position, pub width: u32, pub height: u32}

#[derive(Clone, PartialEq, Debug)]
pub struct CircleCollider{pub position: Position, pub radius: u32}

impl SquareCollider {
    pub fn set_pos(&mut self, position: &Position) {
        self.position = position.clone();
    }

    pub fn is_colliding(&self, other: &SquareCollider) -> bool {
        (self.position.x - other.position.x).abs() <= ((self.width as f64 / 2.0) + (other.width as f64 / 2.0)) &&
        (self.position.y - other.position.y).abs() <= ((self.height as f64 / 2.0) + (other.height as f64 / 2.0))
    }
}

impl CircleCollider {
    pub fn set_pos(&mut self, position: &Position) {
        self.position = position.clone();
    }

    pub fn is_colliding(&self, other: &CircleCollider) -> bool {
        (self.position.x - other.position.x).abs() <= ((self.radius as f64) + (other.radius as f64))
    }

    pub fn collision_angle(&self, other: &CircleCollider) -> f64 {
        0.0
    }
}