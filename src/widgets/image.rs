use elvis_core::{style::ImageSrc, Node};
use elvis_support::RefIntoNode;

/// If you don't want Image playing in background anonymously, just remove its child.
#[derive(RefIntoNode)]
pub struct Image {
    /// Image source
    pub src: ImageSrc,
    /// Image child
    pub child: Node,
}

impl Into<Node> for &Image {
    fn into(self) -> Node {
        Node::default().children(vec![self.child.clone()])
    }
}
