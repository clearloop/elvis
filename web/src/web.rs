use wasm_bindgen::prelude::*;

/// common elvis api
#[wasm_bindgen]
pub struct Elvis;

#[wasm_bindgen]
impl Elvis {
    pub fn render(s: String) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.query_selector("body")?.unwrap();

        body.set_inner_html(&s);
        Ok(())
    }
}
