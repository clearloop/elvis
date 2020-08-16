//! Elvis grid layout
use elvis_core::{derive::Setter, style::GridStyle, Class, Node};
use elvis_derive::IntoNode;

/// `Grid` is quite complex in some way, usually, we just `Grid` our contains.
#[derive(Default, IntoNode, Setter)]
pub struct Grid {
    /// Grid children
    pub children: Vec<Node>,
    /// Grid style
    pub style: GridStyle,
}
