use elvis_core::value::{Colors, Unit};

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
