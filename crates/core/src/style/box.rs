//! Basic style
use crate::{
    style::Border,
    style::Style,
    value::{layouts::Alignment, Color, Unit},
};
use elvis_core_support::Setter;

/// `Container` style
#[derive(Clone, Default, Setter, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContainerStyle {
    /// Container align
    pub align: Alignment,
    /// Container height
    pub height: Unit,
    /// Container width
    pub width: Unit,
    /// Container padding
    pub padding: Unit,
    /// Container margin
    pub margin: Unit,
    /// Container background
    pub background_color: Color,
    /// Container Border
    pub border: Border,
}

impl Into<Vec<Style>> for ContainerStyle {
    fn into(self) -> Vec<Style> {
        let align_style: Vec<Style> = self.align.into();
        let mut styles = vec![
            align_style[0].clone(),
            align_style[1].clone(),
            Style::Height(self.height),
            Style::Width(self.width),
            Style::Padding(self.padding),
            Style::Margin(self.margin),
            Style::BackgroundColor(self.background_color),
        ];
        styles.append(&mut self.border.into());
        styles
    }
}

/// `SizedBox` style
#[derive(Clone, Default, Setter)]
pub struct SizedBoxStyle {
    /// SizedBox height
    pub height: Unit,
    /// SizedBox width
    pub width: Unit,
}

impl Into<Vec<Style>> for SizedBoxStyle {
    fn into(self) -> Vec<Style> {
        vec![Style::Height(self.height), Style::Width(self.width)]
    }
}
