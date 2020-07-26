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

impl ToString for ContainerStyle {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        s += &self.align.to_string();
        s += &format!("height: {};", self.height.to_string());
        s += &format!("width: {};", self.width.to_string());
        s += &format!("padding: {};", self.padding.to_string());
        s += &format!("margin: {};", self.margin.to_string());
        s += &format!("background-color: {};", self.background_color.to_string());
        s
    }
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

impl ToString for SizedBoxStyle {
    fn to_string(&self) -> String {
        format!(
            "height: {}; width: {};",
            self.height.to_string(),
            self.width.to_string()
        )
    }
}

impl Into<[Style; 2]> for SizedBoxStyle {
    fn into(self) -> [Style; 2] {
        [Style::Height(self.height), Style::Width(self.width)]
    }
}
