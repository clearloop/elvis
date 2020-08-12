//! Elvis Style Traits
use crate::{value::Unit, Node, Style};

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
