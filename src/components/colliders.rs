use components::movement::Position;

#[derive(Clone, PartialEq, Debug)]
pub struct Collider{pub position: Position, pub radius: u32}

impl Collider {
    pub fn set_pos(&mut self, position: &Position) {
        self.position = position.clone();
    }

    pub fn is_colliding(&self, other: &Collider) -> bool {
        self.position.distance_to(&other.position) <= (self.radius + other.radius) as f64
    }

    pub fn collision_direction(&self, other: &Collider) -> (f64, f64) {
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