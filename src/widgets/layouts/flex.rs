//! Flex Layout
use elvis_core::{derive::Setter, style::FlexStyle, value::layouts::Alignment, Class, Node};
use elvis_support::IntoNode;

/// `Align` inherits the core usage of Alignment, it's quite simple, just one property.
#[derive(Default, Setter)]
pub struct Align {
    /// Align child
    pub child: Node,
    /// Align style
    pub align: Alignment,
}

impl Into<Node> for Align {
    fn into(self) -> Node {
        Node::default().children(vec![self.child]).style(self.align)
    }
}

/// `Center` is a very nice widget, if your website only have a line of chars, use it!
#[derive(Default, Setter)]
pub struct Center {
    /// Center child
    pub child: Node,
}

impl Into<Node> for Center {
    fn into(self) -> Node {
        Node::default()
            .children(vec![self.child])
            .class(vec![Class::Flex, Class::Center])
    }
}

/// `Col` is the typical flow in html, with flexible enhance.
#[derive(Default, Setter)]
pub struct Col {
    /// Column children
    pub children: Vec<Node>,
}

impl Into<Node> for Col {
    fn into(self) -> Node {
        Node::default()
            .children(self.children)
            .class(vec![Class::Flex, Class::Col])
    }
}

/// This is the Lunatic Widget to Ground Control, 'I`m stepping throw the Window.'
#[derive(Default, IntoNode, Setter)]
pub struct Flex {
    /// Flex child
    pub child: Node,
    /// Flex style
    pub style: FlexStyle,
}

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
#[derive(Default, Setter)]
pub struct Row {
    /// Row children
    pub children: Vec<Node>,
}

impl Into<Node> for Row {
    fn into(self) -> Node {
        Node::default()
            .children(self.children)
            .class(vec![Class::Flex, Class::Row])
    }
}
