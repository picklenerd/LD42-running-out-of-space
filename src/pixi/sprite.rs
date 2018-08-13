use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Positionable, Sizable, Rotatable };

#[derive(Clone, PartialEq, Debug)]
pub struct Sprite {
    pub js_reference: Reference,
}

impl Sprite {
    pub fn new(alias: &str) -> Self {
        let sprite = js! {
            const sprite = new PIXI.Sprite(PIXI.loader.resources[@{alias}].texture);
            sprite.anchor.x = 0.5;
            sprite.anchor.y = 0.5;
            return sprite;
        };
        
        Sprite {
            js_reference: sprite.into_reference().unwrap(),
        }
    }
}

impl JsRef for Sprite {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}

impl Positionable for Sprite {
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

impl Sizable for Sprite {
    fn get_width(&self) -> f64 {
        let x = js! { 
            const me = @{&self.js_reference};
            return me.width;
        };
        x.try_into().unwrap()
    }

    fn set_width(&self, width: f64) {
        js! { @(no_return)
            const rect = @{&self.js_reference};
            rect.width = @{width};
        };
    }

    fn get_height(&self) -> f64 {
        let y = js! { 
            const me = @{&self.js_reference};
            return me.height;
        };
        y.try_into().unwrap()
    }

    fn set_height(&self, height: f64) {
        js! { @(no_return)
            const rect = @{&self.js_reference};
            rect.height = @{height};
        };
    }
}

impl Rotatable for Sprite {
    fn get_angle(&self) -> f64 {
        let angle = js! {
            const me = @{&self.js_reference};
            return me.rotation;
        };
        angle.try_into().unwrap()
    }

    fn set_angle(&self, angle: f64) {
        js! { @(no_return)
            const me = @{&self.js_reference};
            me.rotation = @{angle};
        };
    }
}