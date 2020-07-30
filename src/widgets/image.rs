use elvis_core::{style::ImageSrc, Node};

/// If you don't want Image playing in background anonymously, just remove its child.
pub struct Image {
    /// Image source
    pub src: ImageSrc,
    /// Image child
    pub child: Node,
}
