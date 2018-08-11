use stdweb::{ Reference };
use stdweb::unstable::{ TryFrom };
use stdweb::web::{
    HtmlElement,
};

use super::{ JsRef };

pub struct Application {
    pub js_reference: Reference,
}

impl JsRef for Application {
    fn get_ref(&self) -> &Reference {
        &self.js_reference
    }
}

impl Application {
    pub fn new(width: u32, height: u32, background_color: u32) -> Self {
        let app = js! {
            return new PIXI.Application ({
                width: @{width},
                height: @{height},
                backgroundColor: @{background_color},
            });
        };

        Application {
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
}