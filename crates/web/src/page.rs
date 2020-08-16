use crate::{node, StyleSheet};
use elvis_core::Node;
use std::{cell::RefCell, convert::Into, rc::Rc};
use wasm_bindgen::prelude::*;

/// basic widget without lifecycle nor state
#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Page {
    tree: Node,
    style: Rc<RefCell<StyleSheet>>,
}

impl<N> From<N> for Page
where
    N: Into<Node>,
{
    fn from(n: N) -> Page {
        let mut node: Node = n.into();
        node.idx(&mut vec![]);
        Page {
            tree: node,
            style: Rc::new(RefCell::new(StyleSheet::default())),
        }
    }
}

impl Page {
    /// Render into body element
    pub fn calling(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let dom = window.document().unwrap();

        // set style
        StyleSheet::shared()?;
        self.style()?;

        // set body
        let body = dom.query_selector("body")?.unwrap();
        body.set_inner_html("");
        body.append_child(
            &node::to_element(&Rc::new(RefCell::new(self.tree.clone())), &dom)?.into(),
        )?;

        Ok(())
    }

    /// Shoud update style
    fn style(&mut self) -> Result<bool, JsValue> {
        self.style.borrow_mut().batch(&mut self.tree);
        Ok(self.style.borrow().ser(self.tree.attr.id.to_string())?)
    }
}
