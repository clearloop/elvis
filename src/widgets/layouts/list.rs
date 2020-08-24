use elvis_core::{derive::Setter, Node};

/// `List` is a set of poor orphan children, they don't have any style, just blowing in the wind.
#[derive(Default, Setter)]
pub struct List {
    /// List children
    pub children: Vec<Node>,
}

impl List {
    /// The shortcut of `List::new().children(Vec<Node>)`
    pub fn with(children: Vec<impl Into<Node>>) -> List {
        let mut nodes: Vec<Node> = vec![];
        for n in children {
            nodes.push(n.into());
        }
        Self::new().children(nodes)
    }
}

impl Into<Node> for List {
    fn into(self) -> Node {
        Node::default().children(self.children)
    }
}
