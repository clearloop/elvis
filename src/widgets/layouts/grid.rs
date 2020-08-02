//! Elvis grid layout
use elvis_core::{style::GridStyle, Class, Node};
use elvis_support::IntoNode;

/// `Grid` is quite complex in some way, usually, we just `Grid` our contains.
#[derive(IntoNode)]
pub struct Grid {
    /// Grid children
    pub children: Vec<Node>,
    /// Grid style
    pub style: GridStyle,
}
