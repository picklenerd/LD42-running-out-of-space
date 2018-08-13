pub mod graphics;
pub mod sprite;

use stdweb::{ Reference };
use stdweb::unstable::{ TryFrom };
use stdweb::web::{
    HtmlElement,
};

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

pub trait Sizable {
    fn get_width(&self) -> f64;
    fn get_height(&self) -> f64;
    fn set_width(&self, width: f64);
    fn set_height(&self, height: f64);
    fn set_size(&self, width: f64, height: f64) {
        self.set_width(width);
        self.set_height(height);
    }
    fn set_square_size(&self, size: f64) {
        self.set_size(size, size);
    }
}

pub struct Pixi {
    pub js_reference: Reference,
}

impl JsRef for Pixi {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}

impl Pixi {
    pub fn new(width: u32, height: u32, background_color: u32) -> Self {
        let app = js! {
            return new PIXI.Application ({
                width: @{width},
                height: @{height},
                backgroundColor: @{background_color},
            });
        };

        Self {
            js_reference: app.into_reference().unwrap(),
        }
    }

    pub fn view(&self) -> HtmlElement {
        let view = js! {
            const app = @{&self.js_reference};
            return app.view;
        };
        
        HtmlElement::try_from(view).unwrap()
    }

    pub fn add_child(&self, child: &impl JsRef) {
        js! { @(no_return)
            const app = @{&self.js_reference};
            app.stage.addChild(@{&child.get_ref()});
        };
    }

    pub fn add_child_at(&self, child: &impl JsRef, index: u32) {
        js! { @(no_return)
            const app = @{&self.js_reference};
            app.stage.addChildAt(@{&child.get_ref()}, @{index});
        };
    }

    pub fn remove_child(&self, child: &impl JsRef) {
        js! { @(no_return)
            const app = @{&self.js_reference};
            app.stage.removeChild(@{&child.get_ref()});
        };
    }
}
