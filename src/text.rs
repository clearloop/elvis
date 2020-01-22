use crate::Element;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct TextStyle {
  bold: bool,
  color: String,
  italic: bool,
  size: f64,
}

#[wasm_bindgen]
impl TextStyle {
  #[wasm_bindgen(constructor)]
  pub fn new(bold: bool, color: String, italic: bool, size: f64) -> TextStyle {
    TextStyle {
      bold,
      color,
      italic,
      size,
    }
  }
}

/// Text
///
/// [MDN font CSS][1]
/// [1]: https://developer.mozilla.org/en-US/docs/Web/CSS/font
pub struct Text;

impl Text {
  fn set_text_style(proto: &mut Element, s: Option<TextStyle>) {
    let mut style = TextStyle::default();
    if let Some(s) = s {
      style = s;
    }

    if style.bold {
      proto.set_property("font-weight", "700");
    }

    if style.italic {
      proto.set_property("font-style", "italic");
    }

    proto.set_property("color", &style.color);

    if style.size != 0.0 {
      proto.set_property("font-size", &format!("{}rem", style.size));
    }
  }

  fn proto(tag: &str, text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    let mut proto = Element::new(tag)?.text(text);
    Self::set_text_style(&mut proto, style);
    Ok(proto)
  }

  pub fn plain(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Self::proto("p", text, style)
  }

  pub fn title(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Self::proto("h1", text, style)
  }

  pub fn subtitle(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Self::proto("h2", text, style)
  }

  pub fn headline(text: &str, style: Option<TextStyle>) -> Result<Element, JsValue> {
    Self::proto("h3", text, style)
  }
}
