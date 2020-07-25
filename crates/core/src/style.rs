//! Evlis styles

/// Evlis Style
#[derive(Clone)]
pub enum Style {
    /// Box Height
    Height,
    /// Box Width
    Width,
}

impl ToString for Style {
    fn to_string(&self) -> String {
        match self {
            Style::Height => "height",
            Style::Width => "width",
        }
        .into()
    }
}
