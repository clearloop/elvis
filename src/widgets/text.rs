use crate::{
    value::Unit,
    widgets::{layouts::Container, ListTile},
};
use elvis_core::{
    derive::Setter,
    style::{Border, ContainerStyle, TextStyle},
    Attribute, Node, Style,
};

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[derive(Default, Setter)]
pub struct Text {
    /// Plain text
    #[skip]
    pub text: String,
    /// Text style
    pub style: TextStyle,
}

impl Text {
    /// Set Text
    pub fn text(mut self, s: &str) -> Self {
        self.text = s.into();
        self
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

/// Text Field
#[derive(Default, Setter)]
pub struct TextField {
    /// Leading widget
    #[skip]
    pub leading: Node,
    /// Trailing widget
    #[skip]
    pub trailing: Node,
    /// Plain text
    pub text: Text,
    /// Text style
    pub style: ContainerStyle,
}

impl TextField {
    /// Set leading
    pub fn leading(mut self, l: impl Into<Node>) -> Self {
        self.leading = l.into();
        self
    }

    /// Set trailing
    pub fn trailing(mut self, t: impl Into<Node>) -> Self {
        self.trailing = t.into();
        self
    }
}

impl Into<Node> for TextField {
    fn into(self) -> Node {
        let mut style: Vec<Style> = Border::default().into();
        style.append(&mut vec![
            Style::Width(Unit::Percent(100.0)),
            Style::OutlineWidth(Unit::None(0.0)),
        ]);

        Container::new()
            .style(self.style)
            .child(
                ListTile::new()
                    .leading(self.leading)
                    .text(
                        Into::<Node>::into(self.text)
                            .attr(Attribute::new().tag("input"))
                            .append_style(style),
                    )
                    .trailing(self.trailing),
            )
            .into()
    }
}
