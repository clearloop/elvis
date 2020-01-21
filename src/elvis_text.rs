use crate::ElvisElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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

#[wasm_bindgen]
pub struct TextElement {
  proto: ElvisElement,
}

#[wasm_bindgen]
impl TextElement {
  #[wasm_bindgen(constructor)]
  pub fn new(tag: &str, text: &str, style: TextStyle) -> Result<TextElement, JsValue> {
    let proto = ElvisElement::new(tag)?.text(text);
    if style.bold {
      proto.set_property("font-weight", "700");
    }

    if style.italic {
      proto.set_property("font-style", "italic");
    }

    proto.set_property("color", &style.color);
    proto.set_property("font-size", &format!("{}rem", style.size));

    Ok(TextElement { proto })
  }

  pub fn el(self) -> ElvisElement {
    self.proto
  }
}
