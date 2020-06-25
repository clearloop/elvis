use crate::{
    values::{
        layout::{GridAuto, GridFlow, GridTemplate},
        Unit,
    },
    Tree,
};

/// `Grid` is quite complex in some way, usually, we just `Grid` our contains.
pub struct Grid {
    pub children: Vec<Tree>,
    pub style: GridStyle,
}

/// `Grid` Style
pub struct GridStyle {
    pub col: GridAuto,
    pub col_gap: Unit,
    pub flow: GridFlow,
    pub row: GridAuto,
    pub row_gap: Unit,
    pub template_col: GridTemplate,
    pub template_row: GridTemplate,
}
