use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Positionable };

/*
 * Gonna learn me some macros (some day)
 */
// macro_rules! call_js {
//     ($f:ident($($p:ident),*)) => {
//         js! { @(no_return)
//             const me = @{&self.js_reference};
//             me.$f($($p),*);
//         };
//     };
// }

#[derive(Clone, PartialEq, Debug)]
pub struct Graphics {
    pub js_reference: Reference,
}

impl Graphics {
    pub fn new() -> Self {
        let graphics = js! {
            return new PIXI.Graphics();
        };
        
        Graphics {
            js_reference: graphics.into_reference().unwrap(),
        }
    }

    pub fn line_width(&self, width: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_reference};
            graphics.lineWidth = @{width};
        };
    }

    pub fn line_color(&self, color: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_reference};
            graphics.lineColor = @{color};
        };
    }

    pub fn begin_fill(&self, color: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_reference};
            graphics.beginFill(@{color});
        };
    }

    pub fn draw_rect(&self, x: f64, y: f64, width: u32, height: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_reference};
            graphics.drawRect(@{x}, @{y}, @{width}, @{height});
        };
    }

    pub fn draw_ellipse(&self, x: f64, y: f64, width: u32, height: u32) {
        js! { @(no_return)
            const graphics = @{&self.js_reference};
            graphics.drawEllipse(@{x}, @{y}, @{width}, @{height});
        };
    }
}

impl JsRef for Graphics {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}

impl Positionable for Graphics {
    fn get_x(&self) -> f64 {
        let x = js! { 
            const me = @{&self.js_reference};
            return me.x;
        };
        x.try_into().unwrap()
    }

    fn set_x(&self, new_x: f64) {
        js! { @(no_return)
            const rect = @{&self.js_reference};
            rect.x = @{new_x};
        };
    }

    fn get_y(&self) -> f64 {
        let y = js! { 
            const me = @{&self.js_reference};
            return me.y;
        };
        y.try_into().unwrap()
    }

    fn set_y(&self, new_y: f64) {
        js! { @(no_return)
            const rect = @{&self.js_reference};
            rect.y = @{new_y};
        };
    }

    fn set_position(&self, x: f64, y: f64) {
        js! {@(no_return)
            const rect = @{&self.js_reference};
            rect.x = @{x};
            rect.y = @{y};
        };
    }
}