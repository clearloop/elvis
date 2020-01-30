use crate::Element;
use elvis::{Serde, Text as ElvisText, TextStyle as ElvisTextStyle, Tree};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[wasm_bindgen]
pub struct Text(Element);

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(s: String, style: TextStyle) -> Text {
        let t = ElvisText::new(s, ElvisTextStyle::default());
        Text(Element::new(t.ser()))
    }
}

/// style of `Text`
///
/// `TextStyle` in `elvis-web` is a duplicate implementation to `elvis` `TextStyle`.
#[wasm_bindgen]
pub struct TextStyle(String);

#[wasm_bindgen]
impl TextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(bold: bool) -> TextStyle {
        let css = "".to_string();
        TextStyle(css)
    }
}
