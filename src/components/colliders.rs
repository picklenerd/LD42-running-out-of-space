use components::movement::Position;

#[derive(Clone, PartialEq, Debug)]
pub struct SquareCollider{pub position: Position, pub width: u32, pub height: u32}

impl SquareCollider {
    pub fn set_pos(&mut self, position: &Position) {
        self.position = position.clone();
    }

    pub fn is_colliding(&self, other: &SquareCollider) -> bool {
        (self.position.x - other.position.x).abs() <= ((self.width as f64 / 2.0) + (other.width as f64 / 2.0)) &&
        (self.position.y - other.position.y).abs() <= ((self.height as f64 / 2.0) + (other.height as f64 / 2.0))
    }

    pub fn collision_direction(&self, other: &SquareCollider) -> (f64, f64) {
        if !self.is_colliding(other) {
            (0.0, 0.0)
        } else {
            let x_diff = self.position.x - other.position.x;
            let y_diff = self.position.y - other.position.y;
            
            if y_diff.abs() > x_diff.abs() {
                return (x_diff.signum(), 0.0);
            } else {
                return (0.0, y_diff.signum());
            }
        }
    }
}