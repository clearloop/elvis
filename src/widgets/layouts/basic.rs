//! Basic layout widgets
use crate::{
    widgets::values::{layouts::Alignments, Colors, Unit},
    Node,
};

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
pub struct Container {
    /// Container child
    pub child: Node,
    /// Container style
    pub style: ContainerStyle,
}

/// `Container` style
#[derive(Default)]
pub struct ContainerStyle {
    /// Container align
    pub align: Alignments,
    /// Container height
    pub height: Unit,
    /// Container width
    pub width: Unit,
    /// Container padding
    pub padding: Unit,
    /// Container margin
    pub margin: Unit,
    /// Container background
    pub background_color: Colors,
}

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
pub struct List {
    /// List children
    pub children: Vec<Node>,
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
pub struct SizedBox {
    /// SizedBox child
    pub child: Node,
    /// SizedBox style
    pub style: SizedBoxStyle,
}

/// `SizedBox` style
pub struct SizedBoxStyle {
    /// SizedBox height
    pub height: Unit,
    /// SizedBox width
    pub width: Unit,
}
