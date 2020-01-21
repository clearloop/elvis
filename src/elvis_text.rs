use crate::ElvisElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ITextStyle {
  pub bold: Option<bool>,
  pub italic: Option<bool>,
  pub size: Option<i32>,
}

#[wasm_bindgen]
#[derive(ElvisElement)]
pub struct ElvisText {
  el: ElvisElement,
}

#[wasm_bindgen]
impl ElvisText {
  #[wasm_bindgen(constructor)]
  pub fn new(tag: &str, text: &str, style: Option<ITextStyle>) -> Result<ElvisText, JsValue> {
    let el = ElvisElement::new(tag)?.text(text);
    if let Some(s) = style {
      if let Some(b) = s.bold {
        if b == true {
          el.set_property("fontWeight", "700");
        }
      }

      if let Some(i) = s.italic {
        if i == true {
          el.set_property("fontStyle", "italic");
        }
      }

      if let Some(size) = s.size {
        el.set_property("fontSize", &format!("{}rem", size));
      }
    }

    Ok(ElvisText { el })
  }
}
