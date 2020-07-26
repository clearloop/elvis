//! node opt
use crate::gesture;
use elvis_core::{Class, Node};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

/// Parse class to web string
pub fn parse_class(class: &Class) -> &'static str {
    match class {
        Class::Center => "center",
        Class::Flex => "flex",
        Class::Col => "col",
        Class::Row => "row",
        Class::Empty => "",
    }
}

fn class(classes: &Vec<Class>) -> String {
    let mut r = "".to_string();
    classes.iter().for_each(|c| {
        r.push_str(" ");
        r.push_str(parse_class(c));
    });
    r.trim().into()
}

/// Converts node to element
pub fn to_element(node: &Rc<RefCell<Node>>, dom: &Document) -> Result<Element, JsValue> {
    let this = gesture::bind(node, dom)?;
    this.set_class_name(&class(&node.borrow().class));
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
