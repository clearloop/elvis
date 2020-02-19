use crate::{State, StyleSheet};
use wasm_bindgen::prelude::*;

/// Elvis App entry
#[wasm_bindgen]
pub struct Elvis(State);

#[wasm_bindgen]
impl Elvis {
    #[wasm_bindgen(constructor)]
    pub fn new(s: State) -> Elvis {
        Elvis(s)
    }

    pub fn calling(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.query_selector("html")?.unwrap();

        // set style
        let style = document.create_element("style")?;
        let mut stylesheet = StyleSheet::new();
        stylesheet.0 += &self.0.style();
        style.set_inner_html(&stylesheet.0);
        html.append_child(&style)?;

        // set body
        let body = document.query_selector("body")?.unwrap();
        body.set_inner_html(&self.0.ser());
        Ok(())
    }
}
