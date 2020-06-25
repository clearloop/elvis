use crate::{
    values::{layout::MultiColumnLineStyle, Colors, Unit},
    Tree,
};

/// **Homework**: code a New York Times.
pub struct MultiColumn {
    pub children: Vec<Tree>,
    pub style: MultiColumnStyle,
}

/// `Multicolumn` Style
pub struct MultiColumnStyle {
    pub color: Colors,
    pub count: Unit,
    pub gap: Unit,
    pub style: MultiColumnLineStyle,
}
