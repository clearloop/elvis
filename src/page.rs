use crate::Element;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;
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
    let html = document
      .query_selector("html")
      .expect("should have a html on document")
      .expect("should have a html on document")
      .dyn_into::<HtmlElement>()?;
    let body = document.body().expect("should have a body on document");
    html.style().set_css_text("margin: 0; height: 100%;");
    body.style().set_css_text("margin: 0; height: 100%;");
    body.append_child(&self.child.el())?;
    Ok(())
  }
}
