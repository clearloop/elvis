use crate::{Colors, Unit, Widget};
use elvis::{Text as ElvisText, Tree};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[wasm_bindgen]
pub struct Text(ElvisText);

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String, style: TextStyle) -> Text {
        Text(ElvisText::new(text, style.into()))
    }

    pub fn to_widget(self) -> Widget {
        Widget::new(self.0.into())
    }
}

deref!(Text, ElvisText);
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
