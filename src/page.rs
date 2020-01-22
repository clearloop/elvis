use crate::Element;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Page {
  child: Element,
}

#[wasm_bindgen]
impl Page {
  #[wasm_bindgen(constructor)]
  pub fn new(child: Element) -> Page {
    Page { child }
  }

  pub fn render(self) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("should have a body on document");
    body.append_child(&self.child.el())?;
    Ok(())
  }
}
