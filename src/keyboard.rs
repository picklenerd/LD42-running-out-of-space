use stdweb::{ Reference };
use stdweb::unstable::{ TryInto };

use ::pixi::JsRef;

pub struct Keyboard {
    pub js_reference: Reference,
}

impl Keyboard {
    pub fn new() -> Self {
        let keyboard = js! {
            return new Keyboard();
        };

        Self { 
            js_reference: keyboard.into_reference().unwrap(),
        }
    }

    pub fn key_down(&self, key: &str) -> bool {
        let key_down = js! {
            const keyboard = @{&self.js_reference};
            return keyboard.isKeyDown(@{key});
        };

        key_down.try_into().unwrap()
    }

    pub fn track_key(&self, key_code: &str, alias: &str) {
        js! { @(no_return)
            const keyboard = @{&self.js_reference};
            keyboard.trackKey(@{key_code}, @{alias});
        };
    }
}

impl JsRef for Keyboard {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}