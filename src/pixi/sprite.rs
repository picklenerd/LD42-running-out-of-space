use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use super::{ JsRef, Position };

#[derive(Clone)]
pub struct Sprite {
    pub js_reference: Reference,
}

impl Sprite {
    pub fn new(alias: &str) -> Self {
        let sprite = js! {
            const sprite = new PIXI.Sprite(PIXI.loader.resources[@{alias}].texture);
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

impl Position for Sprite {
    fn get_x(&self) -> i32 {
        let x = js! { 
            const me = @{&self.js_reference};
            return me.x;
        };
        x.try_into().unwrap()
    }

    fn set_x(&self, new_x: i32) {
        js! { @(no_return)
            const rect = @{&self.js_reference};
            rect.x = @{new_x};
        };
    }

    fn get_y(&self) -> i32 {
        let y = js! { 
            const me = @{&self.js_reference};
            return me.y;
        };
        y.try_into().unwrap()
    }

    fn set_y(&self, new_y: i32) {
        js! { @(no_return)
            const rect = @{&self.js_reference};
            rect.y = @{new_y};
        };
    }

    fn set_position(&self, x: i32, y: i32) {
        js! {@(no_return)
            const rect = @{&self.js_reference};
            rect.x = @{x};
            rect.y = @{y};
        };
    }
}