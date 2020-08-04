use elvis_core::{derive::Setter, style::ImageSrc, Node};

/// If you don't want Image playing in background anonymously, just remove its child.
#[derive(Default, Setter)]
pub struct Image {
    /// Image source
    pub src: ImageSrc,
    /// Image child
    pub child: Node,
}

impl Into<Node> for Image {
    fn into(self) -> Node {
        Node::default().children(vec![self.child.clone()])
    }
}
