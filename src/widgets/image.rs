use crate::Tree;

/// If you don't want Image to play in background anonymously, just remove the child field.
pub struct Image {
    pub src: String,
    pub child: Tree,
}

impl Image {
    /// image will auto-fill to the father widget
    pub fn new(src: String, child: Tree) -> Image {
        Image { src, child }
    }
}
