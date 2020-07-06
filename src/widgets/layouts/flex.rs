//! Flex Layout
use crate::{
    widgets::values::{
        layouts::{Alignments, FlexBasis, FlexDirection},
        Unit,
    },
    Node,
};

/// `Align` inherits the core usage of Alignments, it's quite simple, just one property.
pub struct Align<T>
where
    T: Into<Node>,
{
    /// Align child
    pub child: T,
    /// Align style
    pub style: AlignStyle,
}

/// `Align` style
pub struct AlignStyle {
    /// Align value
    pub align: Alignments,
}

/// `Center` is a very nice widget, if your website only have a line of chars, use it!
pub struct Center<T>
where
    T: Into<Node>,
{
    /// Center child
    pub child: T,
}

/// `Col` is the typical flow in html, with flexible enhance.
pub struct Col<T>
where
    T: Into<Node>,
{
    /// Column children
    pub children: Vec<T>,
}

/// This is the Lunatic Widget to Ground Control, 'I`m stepping throw the Window.'
pub struct Flex<T>
where
    T: Into<Node>,
{
    /// Flex child
    pub child: T,
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

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
pub struct Row<T>
where
    T: Into<Node>,
{
    /// Row children
    pub children: Vec<T>,
}
