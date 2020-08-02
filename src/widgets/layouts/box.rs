//! Basic layout widgets
use elvis_core::{
    style::{ContainerStyle, SizedBoxStyle},
    Class, Node,
};
use elvis_support::IntoNode;

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
#[derive(IntoNode)]
pub struct Container {
    /// Container child
    pub child: Node,
    /// Container style
    pub style: ContainerStyle,
}

/// `SizedBox` just has width and height two arguments, we use this component to take some white space usually.
#[derive(IntoNode)]
pub struct SizedBox {
    /// SizedBox child
    pub child: Node,
    /// SizedBox style
    pub style: SizedBoxStyle,
}
