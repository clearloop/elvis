use elvis_core::{derive::Setter, Node};

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
#[derive(Default, Setter)]
pub struct List {
    /// List children
    pub children: Vec<Node>,
}

impl Into<Node> for List {
    fn into(self) -> Node {
        Node::default().children(self.children)
    }
}
