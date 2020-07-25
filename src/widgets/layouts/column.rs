//! Elvis column layout
use elvis_core::{style::MultiColumnStyle, Node};

/// **Homework**: code a New York Times.
pub struct MultiColumn {
    /// Column children
    pub children: Vec<Node>,
    /// Column style
    pub style: MultiColumnStyle,
}
