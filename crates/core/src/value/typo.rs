/// Font Style
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum FontStyle {
    /// Italic Font
    Italic,
    /// Nomal Font
    Normal,
}

impl ToString for FontStyle {
    fn to_string(&self) -> String {
        match self {
            FontStyle::Italic => "italic",
            FontStyle::Normal => "normal",
        }
        .to_string()
    }
}

impl From<&str> for FontStyle {
    fn from(s: &str) -> FontStyle {
        match s.to_lowercase().as_str() {
            "italic" => FontStyle::Italic,
            _ => FontStyle::Normal,
        }
    }
}
