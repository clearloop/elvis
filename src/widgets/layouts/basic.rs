//! Basic layout widgets
use crate::{
    widgets::values::{layouts::Alignments, Colors, Unit},
    Node,
};

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
pub struct Container<T>
where
    T: Into<Node>,
{
    /// Container child
    pub child: T,
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
pub struct List<T>
where
    T: Into<Node>,
{
    /// List children
    pub children: Vec<T>,
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
pub struct SizedBox<T>
where
    T: Into<Node>,
{
    /// SizedBox child
    pub child: T,
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
