use crate::event::EventListener;
use std::{cell::RefCell, rc::Rc};
use elvis_shared::{Node, Gesture};
use std::collections::HashMap;
use web_sys::{Document, Element};
use wasm_bindgen::JsValue;

/// Bind gesture to node
pub fn bind(node: &Rc<RefCell<Node>>, dom: &Document) -> Result<Element, JsValue>{
    let this = dom.create_element(&node.borrow().tag)?;
    if let  Some(gestures) = &node.borrow().gesture {
        for (m, f) in gestures.clone() {
            let state = node.borrow().state.clone();
            EventListener::new(&this, parse_gesture(&m),move |_| {
                if let Some(s) = &state {
                    f(s.clone());
                } else {
                    f(HashMap::new());
                }
            }).forget();
        }
    }

    Ok(this)
}

fn parse_gesture(g: &Gesture) -> &'static str {
    match g {
        Gesture::Tap => "click",
        Gesture::LongTap => "click",
    }
}
