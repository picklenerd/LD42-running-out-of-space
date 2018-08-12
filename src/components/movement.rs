#[derive(Clone, PartialEq, Debug)]
pub struct Position{pub x: f64, pub y: f64}

impl Position {
    pub fn distance_to(&self, other: &Position) -> f64 {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        ((x_diff * x_diff) + (y_diff * y_diff)).sqrt()
    }
    
    pub fn vector_to(&self, other: &Position) -> (f64, f64) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        let angle = y_diff.atan2(x_diff);

        (angle.cos(), angle.sin())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Velocity{pub x: f64, pub y: f64}

#[derive(Clone, PartialEq, Debug)]
pub struct Slowable{pub is_slowed: bool, pub multiplier: f64}

impl Slowable {
    pub fn new(multiplier: f64) -> Self {
        Self {
            is_slowed: false,
            multiplier,
        }
    }

    pub fn set(&mut self, is_slowed: bool) {
        self.is_slowed = is_slowed;
    }
}