//! Flex Layout
use elvis_core::{style::FlexStyle, value::layouts::Alignment, Node};

/// `Align` inherits the core usage of Alignment, it's quite simple, just one property.
pub struct Align {
    /// Align child
    pub child: Node,
    /// Align style
    pub style: Alignment,
}

/// `Center` is a very nice widget, if your website only have a line of chars, use it!
pub struct Center {
    /// Center child
    pub child: Node,
}

/// `Col` is the typical flow in html, with flexible enhance.
pub struct Col {
    /// Column children
    pub children: Vec<Node>,
}

/// This is the Lunatic Widget to Ground Control, 'I`m stepping throw the Window.'
pub struct Flex {
    /// Flex child
    pub child: Node,
    /// Flex style
    pub style: FlexStyle,
}

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
pub struct Row {
    /// Row children
    pub children: Vec<Node>,
}
