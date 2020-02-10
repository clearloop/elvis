use crate::TextStyle;
use elvis::{Image, Text, Tree};
use std::convert::Into;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
/// basic widget without lifecycle nor state
#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Widget(Tree);

impl Widget {
    /// new widget from tree
    pub fn new<W>(tree: W) -> Widget
    where
        W: Into<Tree>,
    {
        Widget(tree.into())
    }
}

#[wasm_bindgen]
impl Widget {
    #[wasm_bindgen(constructor)]
    pub fn constructor() -> Widget {
        Widget::default()
    }
}

impl std::convert::Into<Tree> for Widget {
    fn into(self) -> Tree {
        self.0
    }
}

deref!(Widget, Tree);

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[wasm_bindgen(js_name = "Text")]
pub fn text(text: Option<String>, style: Option<TextStyle>) -> Widget {
    Widget::new(Text::new(
        text.unwrap_or_default(),
        style.unwrap_or_default().into(),
    ))
}

/// If you don't want Image to play in background anonymously, just remove the child field.
///
/// **Note**: It's important to wrap a container outsize the `Image`
#[wasm_bindgen(js_name = "Image")]
pub fn img(src: Option<String>, child: Option<Widget>) -> Widget {
    Widget::new(Image::new(
        src.unwrap_or("".into()),
        child.unwrap_or_default().into(),
    ))
}
