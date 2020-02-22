use crate::{StyleSheet, TextStyle};
use elvis::{Image, Serde, Text, Tree};
use std::convert::Into;
use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};
use wasm_bindgen::prelude::*;

/// basic widget without lifecycle nor state
#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Widget(Tree);
deref!(Widget, Tree);

impl Widget {
    /// new widget from tree
    pub fn new<W>(tree: W) -> Widget
    where
        W: Into<Tree>,
    {
        let mut t = tree.into();
        t.idx(&mut vec![]);
        Widget(t)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
impl Widget {
    pub fn calling(&mut self) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.query_selector("html")?.unwrap();

        // set style
        let style = document.create_element("style")?;
        let mut stylesheet = StyleSheet::new();
        stylesheet.0 += &self.style();
        style.set_inner_html(&stylesheet.0);
        html.append_child(&style)?;

        // set body
        let body = document.query_selector("body")?.unwrap();
        body.set_inner_html(&self.ser());
        Ok(())
    }

    #[wasm_bindgen(constructor)]
    pub fn constructor() -> Widget {
        Widget::default()
    }

    pub fn id(&self) -> String {
        self.attrs.get("id").unwrap_or(&"".to_string()).to_string()
    }

    pub fn patch(&mut self) -> Result<(), JsValue> {
        let id = self.id();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        if let Some(element) = document.query_selector(&format!("#{}", id))? {
            if element.outer_html() != self.ser() {
                element.set_outer_html(&self.ser());
            }
        }
        Ok(())
    }

    pub fn style(&mut self) -> String {
        StyleSheet::batch(self, &mut HashSet::new())
            .trim()
            .to_string()
    }
}

impl std::convert::Into<Tree> for Widget {
    fn into(self) -> Tree {
        self.0
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
  child: Widget;
}
"#;
