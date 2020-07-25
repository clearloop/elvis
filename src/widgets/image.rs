use crate::Node;
use elvis_core::style::ImageSrc;

/// If you don't want Image playing in background anonymously, just remove its child.
pub struct Image {
    /// Image source
    pub src: ImageSrc,
    /// Image child
    pub child: Node,
}
