pub mod application;
pub mod graphics;
pub mod sprite;
pub mod loader;

use stdweb::{ Reference };

pub trait JsRef {
    fn get_ref(&self) -> &Reference;
}

pub trait Positionable {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn set_x(&self, x: f64);
    fn set_y(&self, y: f64);
    fn add_x(&self, n: f64) {
        self.set_x(self.get_x() + n);
    }
    fn add_y(&self, n: f64) {
        self.set_y(self.get_y() + n);
    }
    fn set_position(&self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }
}
