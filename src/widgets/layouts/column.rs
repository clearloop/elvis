//! Elvis column layout
use elvis_core::{derive::Setter, style::MultiColumnStyle, Class, Node};
use elvis_support::IntoNode;

/// **Homework**: code a New York Times.
#[derive(Default, IntoNode, Setter)]
pub struct MultiColumn {
    /// Column children
    pub children: Vec<Node>,
    /// Column style
    pub style: MultiColumnStyle,
}
