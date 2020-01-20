use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct IElvisElementArgs;

#[wasm_bindgen]
pub struct ElvisElement {
  el: web_sys::HtmlElement,
}

#[wasm_bindgen]
impl ElvisElement {
  #[wasm_bindgen(constructor)]
  pub fn new(t: &str) -> Result<ElvisElement, JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");

    Ok(ElvisElement {
      el: document
        .create_element(t)?
        .dyn_into::<web_sys::HtmlElement>()?,
    })
  }

  #[wasm_bindgen(getter)]
  pub fn el(self) -> web_sys::HtmlElement {
    self.el
  }

  pub fn css(self, s: &str) -> Self {
    self.el.style().set_css_text(s);
    self
  }

  pub fn text(self, s: &str) -> Self {
    self.el.set_inner_text(s);
    self
  }
}
