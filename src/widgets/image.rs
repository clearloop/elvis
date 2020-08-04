use elvis_core::{derive::Setter, Attribute, Node};

/// If you don't want Image playing in background anonymously, just remove its child.
#[derive(Default, Setter)]
pub struct Image {
    /// Image source
    pub src: String,
    /// Image child
    pub child: Node,
}

impl Into<Node> for Image {
    fn into(self) -> Node {
        Node::default()
            .children(vec![self.child.clone()])
            .attr(Attribute::new().src(self.src))
    }
}
