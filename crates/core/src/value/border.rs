use elvis_core_support::EnumStyle;

/// The border-style shorthand CSS property sets the line style for
/// all four sides of an element's border.
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
