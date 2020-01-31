use wasm_bindgen::prelude::*;

/// mock web element
#[wasm_bindgen]
pub struct Element(String);

#[wasm_bindgen]
impl Element {
    #[wasm_bindgen(getter)]
    pub fn html(self) -> String {
        self.0
    }

    #[wasm_bindgen(constructor)]
    pub fn new(html: String) -> Element {
        Element(html)
    }
}
