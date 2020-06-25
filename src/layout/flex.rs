//! Flex Layout
use crate::{
    values::{
        layout::{Alignments, FlexBasis, FlexDirection},
        Unit,
    },
    Tree,
};

/// `Align` inherits the core usage of Alignments, it's quite simple, just one property.
pub struct Align {
    pub child: Tree,
    pub style: AlignStyle,
}

/// `Align` style
pub struct AlignStyle {
    pub align: Alignments,
}

/// `Center` is a very nice widget, if your website only have a line of chars, use it!
pub struct Center {
    pub child: Tree,
}

/// `Col` is the typical flow in html, with flexible enhance.
pub struct Col {
    pub children: Vec<Tree>,
}

/// This is the Lunatic Widget to Ground Control, 'I`m stepping throw the Window.'
pub struct Flex {
    pub child: Tree,
    pub style: FlexStyle,
}

/// `Flex` Style
pub struct FlexStyle {
    pub align: Alignments,
    pub basis: FlexBasis,
    pub direction: FlexDirection,
    pub grow: Unit,
    pub order: Unit,
    pub wrap: bool,
}

/// Both `Col` and `Row` are using flex-start, if you want to reverse the children of them, better to work on the list order.
pub struct Row {
    pub children: Vec<Tree>,
}
