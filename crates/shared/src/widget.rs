//! Widget traits
use super::Node;

/// The trait is used for dealing comon UI node and grestures nodes
pub trait Widget {
    fn node(&self) -> Node;
}

impl Widget for Node {
    fn node(&self) -> Node {
        self.clone()
    }
}
