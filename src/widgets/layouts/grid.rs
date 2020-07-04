//! Elvis grid layout
use crate::{
    widgets::values::{
        layouts::{GridAuto, GridFlow, GridTemplate},
        Unit,
    },
    Node,
};

/// `Grid` is quite complex in some way, usually, we just `Grid` our contains.
pub struct Grid {
    /// Grid children
    pub children: Vec<Node>,
    /// Grid style
    pub style: GridStyle,
}

/// `Grid` Style
pub struct GridStyle {
    /// Grid column
    pub col: GridAuto,
    /// Grid column gap
    pub col_gap: Unit,
    /// Grid flow
    pub flow: GridFlow,
    /// Grid row
    pub row: GridAuto,
    /// Grid row gap
    pub row_gap: Unit,
    /// Grid template_column
    pub template_col: GridTemplate,
    /// Grid template_row
    pub template_row: GridTemplate,
}
