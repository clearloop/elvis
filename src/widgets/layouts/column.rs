use crate::{
    widgets::values::{layouts::MultiColumnLineStyle, Colors, Unit},
    Node,
};

/// **Homework**: code a New York Times.
pub struct MultiColumn {
    pub children: Vec<Node>,
    pub style: MultiColumnStyle,
}

/// `Multicolumn` Style
pub struct MultiColumnStyle {
    pub color: Colors,
    pub count: Unit,
    pub gap: Unit,
    pub style: MultiColumnLineStyle,
}
