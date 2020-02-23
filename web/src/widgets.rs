use crate::{StyleSheet, TextStyle};
use elvis::{Image, Serde, Text, Tree};
use std::{cell::RefCell, convert::Into, rc::Rc};
use wasm_bindgen::prelude::*;

/// basic widget without lifecycle nor state
#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Widget {
    tree: Tree,
    style: Rc<RefCell<StyleSheet>>,
}

impl Widget {
    /// new widget from tree
    pub fn new<W>(tree: W) -> Widget
    where
        W: Into<Tree>,
    {
        let mut t = tree.into();
        t.idx(&mut vec![]);
        Widget {
            tree: t,
            style: Rc::new(RefCell::new(StyleSheet::default())),
        }
    }

    pub fn style(&mut self) -> Result<String, JsValue> {
        self.style.borrow_mut().batch(&mut self.tree);
        Ok(self.style.borrow().ser()?)
    }
}

#[wasm_bindgen]
impl Widget {
    pub fn id(&self) -> String {
        self.tree
            .attrs
            .get("id")
            .unwrap_or(&"".to_string())
            .to_string()
    }

    #[wasm_bindgen(js_name = "setIdx")]
    pub fn set_idx(&mut self, id: String) {
        self.tree.attrs.insert("id".to_string(), id);
    }

    pub fn calling(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.query_selector("html")?.unwrap();

        // set style
        StyleSheet::shared()?;
        let style = document.create_element("style")?;
        style.set_inner_html(self.style()?.trim());
        style.set_id(&format!("elvis-style-{}", self.id()));
        html.append_child(&style)?;

        // set body
        let body = document.query_selector("body")?.unwrap();
        body.set_inner_html(&self.tree.ser());
        Ok(())
    }

    #[wasm_bindgen(constructor)]
    pub fn constructor() -> Widget {
        Widget::default()
    }

    pub fn patch(&mut self) -> Result<bool, JsValue> {
        let mut res = false;
        let id = self.id();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let style = self.style()?;
        let cur_style = document
            .query_selector(&format!("#elvis-style-{}", id))?
            .unwrap()
            .inner_html();
        let html = self.tree.ser();
        if let Some(element) = document.query_selector(&format!("#{}", id))? {
            if element.outer_html().ne(&html) || style.ne(&cur_style) {
                res = true;
                element.set_outer_html(&html);
            }
        }
        Ok(res)
    }
}

impl std::convert::Into<Tree> for Widget {
    fn into(self) -> Tree {
        self.tree
    }
}

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[wasm_bindgen(js_name = "Text")]
pub fn text(text: Option<String>, style: Option<TextStyle>) -> Widget {
    Widget::new(Text::new(
        text.unwrap_or_default(),
        style.unwrap_or_default().into(),
    ))
}

/// If you don't want Image playing in background anonymously, just remove the child field.
///
/// **Note**: It's important to wrap a container outsize the `Image`
#[wasm_bindgen(js_name = "Image")]
pub fn img(src: Option<String>, child: Option<Widget>) -> Widget {
    Widget::new(Image::new(
        src.unwrap_or("".into()),
        child.unwrap_or_default().into(),
    ))
}

#[wasm_bindgen(typescript_custom_section)]
const IIMAGE: &'static str = r#"
export interface IImage {
  src: string;
  child: IElvisWidget;
}

export interface IElvisWidget {
  calling: () => void;
}
"#;
