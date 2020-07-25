use crate::Node;

/// If you don't want Image playing in background anonymously, just remove its child.
pub struct Image {
    /// Image source
    pub src: ImageSrc,
    /// Image child
    pub child: Node,
}

/// Image source
pub struct ImageSrc(pub String);

impl ImageSrc {
    /// Serialize source value as bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl ToString for ImageSrc {
    fn to_string(&self) -> String {
        format!("background-image: url({})", self.0)
    }
}
