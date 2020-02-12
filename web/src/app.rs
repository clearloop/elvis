use crate::{StyleSheet, Widget};
use elvis::Serde;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

/// common elvis api
#[wasm_bindgen]
pub struct Elvis {
    home: Widget,
}

#[wasm_bindgen]
impl Elvis {
    #[wasm_bindgen(constructor)]
    pub fn new(home: Widget) -> Elvis {
        Elvis { home }
    }

    pub fn calling(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.query_selector("html")?.unwrap();

        // set style
        let style = document.create_element("style")?;
        let mut stylesheet = StyleSheet::new();
        stylesheet.0 += StyleSheet::batch(&mut self.home, &mut HashSet::new()).trim();
        style.set_inner_html(&stylesheet.0);
        html.append_child(&style)?;

        // set body
        let body = document.query_selector("body")?.unwrap();
        body.set_inner_html(&self.home.ser());
        Ok(())
    }
}
