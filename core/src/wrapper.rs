use crate::Node;

/// Convert widget as a style wrapper
pub trait StyleWrapper {
    /// Converet widget to node
    fn wrap(self) -> Node;
}
