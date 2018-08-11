use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use ::pixi::JsRef;

pub struct Input {
    pub js_reference: Reference,
}

impl Input {
    pub fn new() -> Self {
        let keyboard = js! {
            return new Input();
        };

        Self { 
            js_reference: keyboard.into_reference().unwrap(),
        }
    }

    pub fn is_control_active(&self, control: &str) -> bool {
        let active = js! {
            const input = @{&self.js_reference};
            return input.isControlActive(@{control});
        };

        active.try_into().unwrap()
    }

    pub fn track_key(&self, key_code: &str, alias: &str) {
        js! { @(no_return)
            const input = @{&self.js_reference};
            input.trackKey(@{key_code}, @{alias});
        };
    }

    pub fn track_mouse(&self, button: &str, alias: &str) {
        js! { @(no_return)
            const input = @{&self.js_reference};
            input.trackKey(@{button}, @{alias});
        };
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        let pos = js! {
            const input = @{&self.js_reference};
            return input.mousePosition;
        };

        let pos_vec: Vec<f64> = pos.try_into().unwrap();
        (pos_vec[0], pos_vec[1])
    }
}

impl JsRef for Input {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}