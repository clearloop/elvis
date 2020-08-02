use elvis_core::{style::TextStyle, Node};

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[derive(Debug, Eq, PartialEq)]
pub struct Text {
    /// Plain text
    pub text: String,
    /// Text style
    pub style: TextStyle,
}

impl Text {
    /// New Text
    pub fn new(text: String, style: TextStyle) -> Text {
        Text { text, style }
    }
}

impl Into<Node> for Text {
    fn into(self) -> Node {
        let mut child = Node::default();
        child.attr.tag = "plain".into();
        child.attr.text = self.text.to_string();

        let mut node = Node::default()
            .children(vec![child])
            .style(self.style.clone());

        // Set Tag
        node.attr.tag = "p".into();
        node
    }
}
