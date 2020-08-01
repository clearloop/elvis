use elvis_core_support::EnumStyle;

/// Border Style
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, EnumStyle)]
pub enum BorderStyle {
    /// No Style
    None,
    /// Hidden Border
    Hidden,
    /// Dotted Border
    Dotted,
    /// Dashed Border
    Dashed,
    /// Solid Border
    Solid,
    /// Double Border
    Double,
    /// Groove Style
    Groove,
    /// Ridge Style
    Ridge,
    /// Inset
    Inset,
    /// Outset
    Outset,
}

impl Default for BorderStyle {
    fn default() -> BorderStyle {
        BorderStyle::None
    }
}
