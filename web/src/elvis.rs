use crate::ElvisWidget;
use elvis::Serde;
use wasm_bindgen::prelude::*;

/// common elvis api
#[wasm_bindgen]
pub struct Elvis {
    home: ElvisWidget,
}

#[wasm_bindgen]
impl Elvis {
    #[wasm_bindgen(constructor)]
    pub fn new(widget: ElvisWidget) -> Elvis {
        Elvis { home: widget }
    }

    pub fn calling(&self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.query_selector("body")?.unwrap();

        body.set_inner_html(&self.home.ser());
        Ok(())
    }
}
