use crate::{Colors, Widget};
use elvis::{Text, TextStyle as ElvisTextStyle, Unit};
use wasm_bindgen::prelude::*;

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[wasm_bindgen(js_name = "Text")]
pub fn text(text: Option<String>, style: Option<TextStyle>) -> Widget {
    Widget::new(Text::new(text.unwrap_or_default(), style.unwrap_or_default().into()).into())
}

/// TextStyle Interface
#[wasm_bindgen]
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

#[wasm_bindgen]
impl TextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bold: Option<bool>,
        color: Option<Colors>,
        italic: Option<bool>,
        size: Option<f64>,
        weight: Option<f64>,
        height: Option<f64>,
        stretch: Option<f64>,
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

impl Into<ElvisTextStyle> for TextStyle {
    fn into(self) -> ElvisTextStyle {
        let mut height = Unit::Auto;
        if let Some(u) = self.height {
            height = Unit::Rem(u);
        }

        ElvisTextStyle {
            bold: self.bold.unwrap_or(false),
            color: self.color.unwrap_or_default().into(),
            italic: self.italic.unwrap_or(false),
            size: Unit::Rem(self.size.unwrap_or(1.0)),
            weight: Unit::Rem(self.weight.unwrap_or(1.0)),
            height: height,
            stretch: Unit::Percent(self.stretch.unwrap_or(100.0)),
        }
    }
}
