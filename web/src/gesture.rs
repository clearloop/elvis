use crate::event::EventListener;
use elvis_core::{Gesture, Node};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsValue;
use web_sys::{Document, Element};

/// Bind gesture to node
pub fn bind(node: &Rc<RefCell<Node>>, dom: &Document) -> Result<Element, JsValue> {
    if node.borrow().attr.tag.is_empty() {
        node.borrow_mut().attr.tag = "div".to_string();
    }

    let this = dom.create_element(&node.borrow().attr.tag)?;
    if let Some(gestures) = &node.borrow().gesture {
        for (m, f) in gestures.clone() {
            let state = node.borrow().state.clone();
            EventListener::new(&this, parse_gesture(&m), move |_| {
                if let Some(s) = &state {
                    f(s.clone());
                } else {
                    f(HashMap::new());
                }
            })
            .forget();
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
