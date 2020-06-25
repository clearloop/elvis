use crate::Tree;

/// If you don't want Image playing in background anonymously, just remove its child.
pub struct Image {
    pub src: ImageSrc,
    pub child: Tree,
}

impl Image {
    /// image will auto-fill to the father widget
    pub fn new(src: String, child: Tree) -> Image {
        Image {
            src: ImageSrc(src),
            child,
        }
    }
}

/// Image src
pub struct ImageSrc(pub String);

impl ImageSrc {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
