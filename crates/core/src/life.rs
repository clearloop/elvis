//! Life cycle
use crate::Node;

/// Life cycle trait
pub trait LifeCycle {
    /// Create widget
    fn create(&self) -> Node;
}
