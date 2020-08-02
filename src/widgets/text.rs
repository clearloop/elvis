use crate::widgets::{layouts::Container, ListTile};
use elvis_core::{
    derive::Setter,
    style::{ContainerStyle, TextStyle},
    Attribute, Node,
};

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[derive(Default, Setter)]
pub struct Text {
    /// Plain text
    pub text: String,
    /// Text style
    pub style: TextStyle,
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

#[derive(Default, Setter)]
pub struct TextField {
    /// Leading widget
    pub leading: Node,
    /// Trailing widget
    pub trailing: Node,
    /// Plain text
    pub text: Text,
    /// Text style
    pub style: ContainerStyle,
}

impl Into<Node> for TextField {
    fn into(self) -> Node {
        Container::new()
            .style(self.style)
            .child(
                ListTile::new()
                    .leading(self.leading)
                    .text(Into::<Node>::into(self.text).attr(Attribute::new().tag("input".into())))
                    .trailing(self.trailing)
                    .into(),
            )
            .into()
    }
}
