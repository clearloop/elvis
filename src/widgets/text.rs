use elvis_core::style::TextStyle;

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
