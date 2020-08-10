//! Column Styles
use crate::{
    style::Style,
    value::{layouts::MultiColumnLineStyle, Color, Unit},
};
use elvis_core_support::Setter;

/// `Multicolumn` Style
#[derive(Clone, Copy, Default, Setter)]
pub struct MultiColumnStyle {
    /// Column color
    pub color: Option<Color>,
    /// Column counts
    pub count: Option<Unit>,
    /// Column gap
    pub gap: Option<Unit>,
    /// Column line style
    pub style: Option<MultiColumnLineStyle>,
}

impl Into<Vec<Style>> for MultiColumnStyle {
    fn into(self) -> Vec<Style> {
        let mut styles: Vec<Style> = vec![];
        option_to_style! {
            styles, [
                (ColumnCount, self.count),
                (ColumnGap, self.gap),
                (ColumnRuleColor, self.color),
                (ColumnRuleStyle, self.style),
            ],
        }

        styles
    }
}
