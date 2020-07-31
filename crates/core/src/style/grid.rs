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

impl Into<[Style; 7]> for GridStyle {
    fn into(self) -> [Style; 7] {
        [
            Style::GridAutoColumns(self.col),
            Style::GridAutoRows(self.row),
            Style::GridAutoFlow(self.flow),
            Style::GridColumnGap(self.col_gap),
            Style::GridRowGap(self.row_gap),
            Style::GridTemplateColumns(self.template_col),
            Style::GridTemplateRows(self.template_row),
        ]
    }
}
