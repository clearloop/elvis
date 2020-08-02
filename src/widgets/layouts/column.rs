//! Elvis column layout
use elvis_core::{style::MultiColumnStyle, Class, Node};
use elvis_support::IntoNode;

/// **Homework**: code a New York Times.
#[derive(IntoNode)]
pub struct MultiColumn {
    /// Column children
    pub children: Vec<Node>,
    /// Column style
    pub style: MultiColumnStyle,
}
