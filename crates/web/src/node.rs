//! node opt
use crate::{gesture, style};
use elvis_core::{Class, Node};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

fn parse_class(classes: &[Class], id: &str) -> String {
    let mut r = "".to_string();
    classes.iter().for_each(|c| {
        r.push_str(" ");
        r.push_str(style::parse_class(c));
    });

    // push id
    r.push_str(" ");
    r.push_str(id);

    // trim
    r.trim().into()
}

/// Converts node to element
pub fn to_element(node: &Rc<RefCell<Node>>, dom: &Document) -> Result<Element, JsValue> {
    let this = gesture::bind(node, dom)?;
    let class = parse_class(&node.borrow().class, &node.borrow().attr.id);
    if !class.is_empty() {
        this.set_class_name(&class);
    }

    if node.borrow().tag == "plain" {
        let p = dom.create_element("p")?;
        p.set_inner_html(&node.borrow().attr.text);
        return Ok(p);
    }

    for child in node.borrow().children.iter() {
        this.append_child(&to_element(child, dom)?.into())?;
    }

    Ok(this)
}
