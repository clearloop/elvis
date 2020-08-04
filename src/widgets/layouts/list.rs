use elvis_core::{derive::Setter, Class, Node};
use elvis_support::IntoNode;

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
#[derive(Default, IntoNode, Setter)]
pub struct List {
    /// List children
    pub children: Vec<Node>,
}
