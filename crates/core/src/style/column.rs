//! Column Styles
use crate::{
    style::Style,
    value::{layouts::MultiColumnLineStyle, Colors, Unit},
};
use elvis_core_support::Setter;

/// `Multicolumn` Style
#[derive(Clone, Copy, Default, Setter)]
pub struct MultiColumnStyle {
    /// Column color
    pub color: Colors,
    /// Column counts
    pub count: Unit,
    /// Column gap
    pub gap: Unit,
    /// Column line style
    pub style: MultiColumnLineStyle,
}

impl Into<Vec<Style>> for MultiColumnStyle {
    fn into(self) -> Vec<Style> {
        vec![
            Style::ColumnCount(self.count),
            Style::ColumnGap(self.gap),
            Style::ColumnRuleColor(self.color),
            Style::ColumnRuleStyle(self.style),
        ]
    }
}
