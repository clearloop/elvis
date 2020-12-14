//! Grid Style
use crate::{
    style::Style,
    value::{
        layouts::{GridAuto, GridFlow, GridTemplate},
        Unit,
    },
};
use elvis_core_support::Setter;

/// `Grid` Style
#[derive(Default, Clone, Setter)]
pub struct GridStyle {
    /// Grid column
    pub col: Option<GridAuto>,
    /// Grid column gap
    pub col_gap: Option<Unit>,
    /// Grid flow
    pub flow: Option<GridFlow>,
    /// Grid row
    pub row: Option<GridAuto>,
    /// Grid row gap
    pub row_gap: Option<Unit>,
    /// Grid template_column
    pub template_col: Option<GridTemplate>,
    /// Grid template_row
    pub template_row: Option<GridTemplate>,
}

impl Into<Vec<Style>> for GridStyle {
    fn into(self) -> Vec<Style> {
        let mut styles: Vec<Style> = vec![];
        option_to_style! {
            styles, [
                (GridAutoColumns, self.col),
                (GridAutoRows, self.row),
                (GridAutoFlow, self.flow),
                (GridColumnGap, self.col_gap),
                (GridRowGap, self.row_gap),
                (GridTemplateColumns, self.template_col),
                (GridTemplateRows, self.template_row),
            ],
        }
        styles
    }
}
