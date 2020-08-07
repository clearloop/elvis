use crate::widgets::StyleWrapper;
use elvis_core::{
    derive::{Setter, Wrapper},
    style::ContainerStyle,
    Class, Node,
};
use elvis_support::IntoNode;

/// To be honest, `Container` is a part of Flex family, but he is too brilliant to stay in Flex family, Layout calls him.
#[derive(Default, IntoNode, Setter, Wrapper)]
pub struct Container {
    /// Container child
    pub child: Node,
    /// Container style
    pub style: ContainerStyle,
}
