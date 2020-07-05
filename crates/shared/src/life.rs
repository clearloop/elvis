//! Life cycle
use crate::Node;

/// Life cycle trait
pub trait LifeCycle<T>
where
    T: Into<Node>,
{
    /// Create widget
    fn create(&self) -> T;
    // Render node to target platform
    // fn render(&mut self) -> T;
}
