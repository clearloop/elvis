//! Flex Layout
use elvis_core::{style::FlexStyle, value::layouts::Alignment, Class, Node};
use elvis_support::{IntoNode, RefIntoNode};

/// `Align` inherits the core usage of Alignment, it's quite simple, just one property.
#[derive(IntoNode)]
pub struct Align {
    /// Align child
    pub child: Node,
    /// Align style
    pub style: Alignment,
}

/// `Center` is a very nice widget, if your website only have a line of chars, use it!
#[derive(RefIntoNode)]
pub struct Center {
    /// Center child
    pub child: Node,
}

impl Into<Node> for &Center {
    fn into(self) -> Node {
        Node::default()
            .children(vec![self.child.clone()])
            .class(&mut vec![Class::Flex, Class::Center])
    }
}

/// `Col` is the typical flow in html, with flexible enhance.
#[derive(IntoNode)]
pub struct Col {
    /// Column children
    pub children: Vec<Node>,
}

/// This is the Lunatic Widget to Ground Control, 'I`m stepping throw the Window.'
#[derive(IntoNode)]
pub struct Flex {
    /// Flex child
    pub child: Node,
    /// Flex style
    pub style: FlexStyle,
}

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
#[derive(IntoNode)]
pub struct Row {
    /// Row children
    pub children: Vec<Node>,
}
