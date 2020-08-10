//! Widget Styles
use crate::{
    style::Style,
    value::{Color, FontFamily, FontStyle, TextAlign, Unit},
};
use elvis_core_support::Setter;

/// style of `Text`
#[derive(Clone, Default, Debug, Eq, PartialEq, Setter)]
pub struct TextStyle {
    /// Bold text
    pub bold: bool,
    /// The color of the text
    pub color: Option<Color>,
    /// Italic text
    pub italic: bool,
    /// Text size
    pub size: Option<Unit>,
    /// Text weight
    pub weight: Option<Unit>,
    /// Text height
    pub height: Option<Unit>,
    /// Text stretch
    pub stretch: Option<Unit>,
    /// Font Family
    pub family: Option<FontFamily>,
    /// Text Align
    pub align: Option<TextAlign>,
}

impl Into<Vec<Style>> for TextStyle {
    fn into(mut self) -> Vec<Style> {
        let mut styles: Vec<Style> = vec![];
        if self.italic {
            styles.push(Style::FontStyle(FontStyle::Normal));
        }

        if self.bold {
            self.weight = Some(Unit::None(700.0));
        }

        option_to_style! {
            styles, [
                (Color, self.color),
                (FontWeight, self.weight),
                (FontSize, self.size),
                (FontStretch, self.stretch),
                (LineHeight, self.height),
                (FontFamily, self.family),
                (TextAlign, self.align),
            ],
        }

        styles
    }
}
