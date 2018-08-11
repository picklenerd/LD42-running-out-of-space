pub mod application;
pub mod graphics;
pub mod sprite;
pub mod loader;

use stdweb::{ Reference };

pub trait JsRef {
    fn get_ref(&self) -> &Reference;
}

pub trait Position {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn set_x(&self, x: i32);
    fn set_y(&self, y: i32);
    fn add_x(&self, n: i32) {
        self.set_x(self.get_x() + n);
    }
    fn add_y(&self, n: i32) {
        self.set_y(self.get_y() + n);
    }
    fn set_position(&self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }
}
