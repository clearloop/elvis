//! Basic style
use crate::{
    style::Border,
    style::Style,
    value::{layouts::Alignment, BoxShadow, Color, Unit, VecUnit},
};
use elvis_core_support::Setter;

/// `Container` style
#[derive(Clone, Default, Setter, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContainerStyle {
    /// Container align
    pub align: Alignment,
    /// Container height
    pub height: Unit,
    /// SizedBox Max Height
    pub max_height: Unit,
    /// SizedBox Max Width
    pub max_width: Unit,
    /// Container width
    pub width: Unit,
    /// Container padding
    pub padding: VecUnit,
    /// Container margin
    pub margin: VecUnit,
    /// Container background
    pub background_color: Color,
    /// Container Border
    pub border: Border,
    /// Box Shadow
    pub shadow: BoxShadow,
}

impl Into<Vec<Style>> for ContainerStyle {
    fn into(self) -> Vec<Style> {
        let align_style: Vec<Style> = self.align.into();
        let mut styles = vec![
            align_style[0].clone(),
            align_style[1].clone(),
            Style::Height(self.height),
            Style::Width(self.width),
            Style::MaxHeight(self.max_height),
            Style::MaxWidth(self.max_width),
            Style::Padding(self.padding),
            Style::Margin(self.margin),
            Style::BackgroundColor(self.background_color),
            Style::BoxShadow(self.shadow),
        ];
        styles.append(&mut self.border.into());
        styles
    }
}
