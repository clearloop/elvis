use crate::{Colors, Element, Unit};
use elvis::{Serde, Text as ElvisText};
use wasm_bindgen::prelude::*;

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[wasm_bindgen]
pub struct Text(Element);

#[wasm_bindgen]
impl Text {
    /// serialze `Textstyle` to String
    #[wasm_bindgen(getter)]
    pub fn html(self) -> String {
        self.0.html()
    }

    /// generate a new Text
    #[wasm_bindgen(constructor)]
    pub fn new(s: String, style: TextStyle) -> Text {
        let t = ElvisText::new(s, style.into());
        Text(Element::new(t.ser()))
    }
}

/// style of `Text`
///
/// `TextStyle` in `elvis-web` is a duplicate implementation to `elvis` `TextStyle`.
#[wasm_bindgen]
pub struct TextStyle {
    pub bold: bool,
    pub color: Colors,
    pub italic: bool,
    pub size: Unit,
    pub weight: Unit,
    pub height: Unit,
    pub stretch: Unit,
}

#[wasm_bindgen]
impl TextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bold: bool,
        color: Colors,
        italic: bool,
        size: Unit,
        weight: Unit,
        height: Unit,
        stretch: Unit,
    ) -> TextStyle {
        TextStyle {
            bold,
            color,
            italic,
            size,
            weight,
            height,
            stretch,
        }
    }
}
