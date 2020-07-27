//! Widget Styles
use crate::{
    style::Style,
    value::{Colors, Unit},
};

/// style of `Text`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextStyle {
    /// Bold text
    pub bold: bool,
    /// The color of the text
    pub color: Colors,
    /// Italic text
    pub italic: bool,
    /// Text size
    pub size: Unit,
    /// Text weight
    pub weight: Unit,
    /// Text height
    pub height: Unit,
    /// Text stretch
    pub stretch: Unit,
}

impl Default for TextStyle {
    fn default() -> TextStyle {
        TextStyle {
            bold: true,
            color: Colors::Pink,
            italic: true,
            size: Unit::Rem(2.0),
            weight: Unit::None(400.0),
            height: Unit::Rem(1.0),
            stretch: Unit::Percent(100.0),
        }
    }
}

impl ToString for TextStyle {
    fn to_string(&self) -> String {
        format!(
            "color: {}; font-weight: {}; font-style: {}; font-size: {}; font-stretch: {}; line-height: {};",
            self.color.to_string(),
            if self.bold {
                "700".into()
            } else {
                self.weight.to_string()
            },
            if self.italic {
                "italic"
            } else {
                "normal"
            },
            self.size.to_string(),
            self.stretch.to_string(),
            self.height.to_string(),
        )
    }
}

impl Into<Vec<Style>> for TextStyle {
    fn into(self) -> Vec<Style> {
        vec![
            Style::Color(self.color),
            Style::FontWeight(self.weight),
            Style::FontSize(self.size),
            Style::FontStretch(self.stretch),
            Style::LineHeight(self.height),
        ]
    }
}

/// Image source
pub struct ImageSrc(pub String);

impl ImageSrc {
    /// Serialize source value as bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl ToString for ImageSrc {
    fn to_string(&self) -> String {
        format!("background-image: url({})", self.0)
    }
}
