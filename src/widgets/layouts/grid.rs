//! Elvis grid layout
use elvis_core::{style::GridStyle, Node};

/// `Grid` is quite complex in some way, usually, we just `Grid` our contains.
pub struct Grid {
    /// Grid children
    pub children: Vec<Node>,
    /// Grid style
    pub style: GridStyle,
}
