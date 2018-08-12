use stdweb::Reference;

use super::JsRef;

pub struct Layer {
    pub js_reference: Reference,
}

impl Layer {
    pub fn new() -> Self {
        let layer = js! {
            const layer = new PIXI.display.Layer();
            layer.group.enableSort = true;
            return layer;
        };

        Self {
            js_reference: layer.into_reference().unwrap(),
        }
    }

    pub fn from_group(&self, group: &DisplayGroup) -> Self {
        let layer = js! {
            const group = @{group.get_ref()};
            return new PIXI.display.Layer(group);
        };

        Self {
            js_reference: layer.into_reference().unwrap(),
        }
    }
}

impl JsRef for Layer {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}

pub struct DisplayGroup {
    pub js_reference: Reference,
}

impl DisplayGroup {
    pub fn new(z_index: i32, sort: bool) -> Self {
        let display_group = js! {
            return new PIXI.display.Group(@{z_index}, @{sort});
        };

        Self {
            js_reference: display_group.into_reference().unwrap(),
        }
    }
}

impl JsRef for DisplayGroup {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}