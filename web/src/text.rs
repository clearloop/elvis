use crate::{Colors, Widget};
use elvis::{Text, TextStyle as ElvisTextStyle, Unit};
use wasm_bindgen::prelude::*;

//// `Text` might be the most popular spider from Mars,
//// Does it know the Great Ziggy Stardust?
#[wasm_bindgen(js_name = "Text")]
pub fn text(text: Option<String>, style: Option<TextStyle>) -> Widget {
    Widget::new(Text::new(text.unwrap_or_default(), style.unwrap_or_default().into()).into())
}

/// TextStyle Interface
#[wasm_bindgen(plain_object)]
#[derive(Default)]
pub struct TextStyle {
    pub bold: Option<bool>,
    pub color: Option<Colors>,
    pub italic: Option<bool>,
    pub size: Option<f64>,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub stretch: Option<f64>,
}

impl Into<ElvisTextStyle> for TextStyle {
    fn into(self) -> ElvisTextStyle {
        ElvisTextStyle {
            bold: self.bold.unwrap_or(false),
            color: self.color.unwrap_or_default().into(),
            italic: self.italic.unwrap_or(false),
            size: Unit::Rem(self.size.unwrap_or(1.0)),
            weight: Unit::Rem(self.weight.unwrap_or(1.0)),
            height: Unit::Rem(self.height.unwrap_or(1.0)),
            stretch: Unit::Percent(self.stretch.unwrap_or(100.0)),
        }
    }
}
