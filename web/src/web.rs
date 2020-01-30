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

/// mock web element
#[wasm_bindgen]
pub struct Element(String);

#[wasm_bindgen]
impl Element {
    #[wasm_bindgen(constructor)]
    pub fn new(html: String) -> Element {
        Element(html)
    }
}
