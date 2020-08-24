use elvis_core::{derive::Setter, Node};

/// Link Widget
#[derive(Default, Setter)]
pub struct Link {
    child: Node,
    href: String,
}

impl Link {
    /// New Link with child
    pub fn with(node: impl Into<Node>) -> Self {
        let link = Link::new();
        link.child(node.into())
    }
}

impl Into<Node> for Link {
    fn into(self) -> Node {
        let mut node = Node::default().children(vec![self.child]);
        node.attr.tag = "a".into();
        node.attr.href = self.href;
        node
    }
}
