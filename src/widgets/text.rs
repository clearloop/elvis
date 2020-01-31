use crate::{Colors, Serde, Unit};
/// `Text` might be the most popular spider from Mars,
/// Does it know the Great Ziggy Stardust?
#[derive(Debug, Eq, PartialEq)]
pub struct Text {
    pub text: String,
    pub style: TextStyle,
}

impl Text {
    pub fn new(text: String, style: TextStyle) -> Text {
        Text { text, style }
    }
}

/// style of `Text`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextStyle {
    pub bold: bool,
    pub color: Colors,
    pub italic: bool,
    pub size: Unit,
    pub weight: Unit,
    pub height: Unit,
    pub stretch: Unit,
}

impl TextStyle {
    pub fn new(
        bold: bool,
        color: Colors,
        italic: bool,
        size: Unit,
        weight: Unit,
        height: Unit,
        stretch: Unit,
    ) -> TextStyle {
        TextStyle {
            bold,
            color,
            italic,
            size,
            weight,
            height,
            stretch,
        }
    }
}

impl Default for TextStyle {
    fn default() -> TextStyle {
        TextStyle {
            bold: true,
            color: Colors::Pink,
            italic: true,
            size: Unit::Rem(42.0),
            weight: Unit::None(400.0),
            height: Unit::Rem(1.0),
            stretch: Unit::Percent(100.0),
        }
    }
}

#[allow(unused_imports)]
use crate::features::web::text;
