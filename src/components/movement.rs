#[derive(Clone, PartialEq, Debug)]
pub struct Position{pub x: f64, pub y: f64}

#[derive(Clone, PartialEq, Debug)]
pub struct Velocity{pub x: f64, pub y: f64}

impl Position {
    pub fn vector_to(&self, other: &Position) -> (f64, f64) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        let angle = y_diff.atan2(x_diff);

        (angle.cos(), angle.sin())
    }
}