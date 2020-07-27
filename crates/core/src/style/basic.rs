//! Basic style
use crate::{
    style::Style,
    value::{layouts::Alignments, Colors, Unit},
};

/// `Container` style
#[derive(Clone, Default)]
pub struct ContainerStyle {
    /// Container align
    pub align: Alignments,
    /// Container height
    pub height: Unit,
    /// Container width
    pub width: Unit,
    /// Container padding
    pub padding: Unit,
    /// Container margin
    pub margin: Unit,
    /// Container background
    pub background_color: Colors,
}

impl Into<[Style; 7]> for ContainerStyle {
    fn into(self) -> [Style; 7] {
        let [items, content]: [Style; 2] = self.align.into();
        [
            items,
            content,
            Style::Height(self.height),
            Style::Width(self.width),
            Style::Padding(self.padding.into()),
            Style::Margin(self.margin.into()),
            Style::BackgroundColor(self.background_color),
        ]
    }
}

/// `SizedBox` style
#[derive(Clone, Default)]
pub struct SizedBoxStyle {
    /// SizedBox height
    pub height: Unit,
    /// SizedBox width
    pub width: Unit,
}

impl Into<[Style; 2]> for SizedBoxStyle {
    fn into(self) -> [Style; 2] {
        [Style::Height(self.height), Style::Width(self.width)]
    }
}
