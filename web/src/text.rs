use crate::{Colors, Unit, Widget};
use elvis::Text;
use wasm_bindgen::prelude::*;

//// `Text` might be the most popular spider from Mars,
//// Does it know the Great Ziggy Stardust?
#[wasm_bindgen(js_name = "Text")]
pub fn text(text: String, style: TextStyle) -> Widget {
    Widget::new(Text::new(text, style.into()).into())
}

/// style of `Text`
///
/// `TextStyle` in `elvis-web` is a duplicate implementation to `elvis` `TextStyle`.
#[wasm_bindgen(plain_object)]
pub struct TextStyle {
    pub bold: bool,
    pub color: Colors,
    pub italic: bool,
    pub size: Unit,
    pub weight: Unit,
    pub height: Unit,
    pub stretch: Unit,
}
