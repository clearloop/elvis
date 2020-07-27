//! Column Styles
use crate::{
    style::Style,
    value::{layouts::MultiColumnLineStyle, Colors, Unit},
};

/// `Multicolumn` Style
#[derive(Clone, Default)]
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

impl Into<[Style; 4]> for MultiColumnStyle {
    fn into(self) -> [Style; 4] {
        [
            Style::ColumnCount(self.count),
            Style::ColumnGap(self.gap),
            Style::ColumnRuleColor(self.color),
            Style::ColumnRuleStyle(self.style),
        ]
    }
}
