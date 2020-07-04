use crate::widgets::values::{Colors, Unit};

/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[derive(Debug, Eq, PartialEq)]
pub struct Text {
    /// Plain text
    pub text: String,
    /// Text style
    pub style: TextStyle,
}

impl Text {
    /// New Text
    pub fn new(text: String, style: TextStyle) -> Text {
        Text { text, style }
    }
}

/// style of `Text`
#[derive(Clone, Debug, Eq, PartialEq)]
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
