//! Elvis Style Traits
use crate::{
    value::{Unit, VecUnit},
    Node, Style,
};

/// Width Trait
pub trait Width {
    /// Impl width style for node
    fn width(self, value: Unit) -> Node;
}

impl<T> Width for T
where
    T: Into<Node>,
{
    fn width(self, value: Unit) -> Node {
        let mut node: Node = self.into();
        node.style.push(Style::Width(value));
        node
    }
}

/// Margin Trait
pub trait Margin {
    /// Impl margin for node
    fn margin(self, value: VecUnit) -> Node;
}

impl<T> Margin for T
where
    T: Into<Node>,
{
    fn margin(self, value: VecUnit) -> Node {
        let mut node: Node = self.into();
        node.style.push(Style::Margin(value));
        node
    }
}
