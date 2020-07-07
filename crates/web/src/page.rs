use crate::StyleSheet;
use elvis_shared::{Node, Serde};
use std::{cell::RefCell, convert::Into, rc::Rc};
use wasm_bindgen::prelude::*;

/// basic widget without lifecycle nor state
#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Page {
    tree: Node,
    style: Rc<RefCell<StyleSheet>>,
}

impl Page {
    /// new widget from tree
    pub fn new<W>(tree: W) -> Page
    where
        W: Into<Node>,
    {
        let mut t = tree.into();
        t.idx(&mut vec![]);
        Page {
            tree: t,
            style: Rc::new(RefCell::new(StyleSheet::default())),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl Page {
    /// Get widget id
    pub fn id(&self) -> String {
        self.tree
            .attrs
            .get("id")
            .unwrap_or(&"".to_string())
            .to_string()
    }

    /// Set widget id
    #[wasm_bindgen(js_name = "setIdx")]
    pub fn set_idx(&mut self, id: String) {
        self.tree.attrs.insert("id".to_string(), id);
    }

    /// Shoud update style
    pub fn style(&mut self) -> Result<bool, JsValue> {
        self.style.borrow_mut().batch(&mut self.tree);
        Ok(self.style.borrow().ser(self.id())?)
    }

    /// Render into body element
    pub fn calling(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // set style
        StyleSheet::shared()?;
        self.style()?;

        // set body
        let body = document.query_selector("body")?.unwrap();
        body.set_inner_html(&self.tree.ser());
        Ok(())
    }

    /// New widget from Javascript
    #[wasm_bindgen(constructor)]
    pub fn constructor() -> Page {
        Page::default()
    }

    /// Update dom tree
    pub fn patch(&mut self) -> Result<bool, JsValue> {
        let mut res = self.style()?;
        let html = self.tree.ser();
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(element) = document.query_selector(&format!("#{}", self.id()))? {
            if element.outer_html().ne(&html) {
                res = true;
                element.set_outer_html(&html);
            }
        }
        Ok(res)
    }
}

impl std::convert::Into<Node> for Page {
    fn into(self) -> Node {
        self.tree
    }
}
