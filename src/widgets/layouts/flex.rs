//! Flex Layout
use elvis_core::{
    value::{
        layouts::{Alignments, FlexBasis, FlexDirection},
        Unit,
    },
    Node,
};

/// `Align` inherits the core usage of Alignments, it's quite simple, just one property.
pub struct Align {
    /// Align child
    pub child: Node,
    /// Align style
    pub style: AlignStyle,
}

/// `Align` style
pub struct AlignStyle {
    /// Align value
    pub align: Alignments,
}

impl ToString for AlignStyle {
    fn to_string(&self) -> String {
        self.align.to_string()
    }
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

/// `Flex` Style
pub struct FlexStyle {
    /// Flex align
    pub align: Alignments,
    /// Flex basis
    pub basis: FlexBasis,
    /// Flex direction
    pub direction: FlexDirection,
    /// Flex grow
    pub grow: Unit,
    /// Flex order
    pub order: Unit,
    /// Flex wrap
    pub wrap: bool,
}

impl ToString for FlexStyle {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        s += &self.align.to_string();
        s += &self.basis.to_string();
        s += &self.direction.to_string();
        s += &format!("flex-grow: {};", self.grow.to_string());
        s += &format!("flex-order: {};", self.order.to_string());
        s += &format!("wrap: {};", if self.wrap { "wrap" } else { "no-wrap" });
        s
    }
}

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
pub struct Row {
    /// Row children
    pub children: Vec<Node>,
}
