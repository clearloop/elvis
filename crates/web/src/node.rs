//! node opt
use elvis_shared::Node;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

/// Converts node to element
pub fn to_element(node: &Rc<RefCell<Node>>, dom: &Document) -> Result<Element, JsValue> {
    let this = dom.create_element(&node.borrow().tag)?;
    this.set_class_name(node.borrow().attrs.get("class").unwrap_or(&"".into()));
    this.set_id(node.borrow().attrs.get("id").unwrap_or(&"".into()));
    if node.borrow().tag == "plain" {
        let p = dom.create_element("p")?;
        p.set_inner_html(node.borrow().attrs.get("text").unwrap_or(&"".into()));
        return Ok(p);
    }

    for child in node.borrow().children.iter() {
        this.append_child(&to_element(child, dom)?.into())?;
    }

    Ok(this)
}
