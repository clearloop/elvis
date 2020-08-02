use elvis_core::{Class, Node};
use elvis_support::IntoNode;

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
#[derive(IntoNode)]
pub struct List {
    /// List children
    pub children: Vec<Node>,
}
