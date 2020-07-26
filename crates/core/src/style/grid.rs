//! Grid Style
use crate::{
    style::Style,
    value::{
        layouts::{GridAuto, GridFlow, GridTemplate},
        Unit,
    },
};

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

impl ToString for GridStyle {
    fn to_string(&self) -> String {
        let mut ss = "".to_string();

        ss.push_str(&format!("grid-auto-columns: {};", self.col.to_string()));
        ss.push_str(&format!("grid-auto-flow: {};", self.flow.to_string()));
        ss.push_str(&format!("grid-auto-rows: {};", self.row.to_string()));
        ss.push_str(&format!("grid-column-gap: {};", self.col_gap.to_string()));
        ss.push_str(&format!("grid-row-gap: {};", self.row_gap.to_string()));
        ss.push_str(&format!(
            "grid-template-columns: {};",
            self.template_col.to_string()
        ));
        ss.push_str(&format!(
            "grid-template-rows: {};",
            self.template_row.to_string()
        ));
        ss
    }
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
