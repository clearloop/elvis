use crate::{Parser, Tree, Unit};
use std::collections::HashMap;
/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
pub struct Text {
    text: String,
    style: TextStyle,
}

impl Text {
    pub fn new(text: String, style: TextStyle) -> Text {
        Text { text, style }
    }

    pub fn ser(self) -> String {
        let mut m = HashMap::<&'static str, &'static str>::new();
        m.insert("text", Box::leak(Box::new(self.text)));

        let t = Tree::new(m, vec![], None, "plain").borrow().to_owned();
        t.ser()
    }
}

/// style of `Text`
#[derive(Default)]
pub struct TextStyle {
    pub bold: bool,
    pub color: bool,
    pub italic: bool,
    pub size: Unit,
    pub weight: Unit,
    pub height: Unit,
    pub stretch: Unit,
}

impl TextStyle {
    pub fn new(
        bold: bool,
        color: bool,
        italic: bool,
        size: Unit,
        weight: Unit,
        height: Unit,
        stretch: Unit,
    ) -> TextStyle {
        TextStyle {
            bold,
            color,
            italic,
            size,
            weight,
            height,
            stretch,
        }
    }
}
