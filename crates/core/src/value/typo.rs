use elvis_core_support::EnumStyle;

/// Text Align
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, EnumStyle)]
pub enum TextAlign {
    /// Align Center
    Center,
    /// Align Left
    Left,
    /// Align Right
    Right,
    /// Align Justify
    Juistyfy,
    /// Align Start
    Start,
    /// Align End
    End,
    /// Align Inherit
    Inherit,
    /// Align Initial
    Initial,
    /// Unset
    Unset,
}

impl Default for TextAlign {
    fn default() -> TextAlign {
        TextAlign::Center
    }
}
