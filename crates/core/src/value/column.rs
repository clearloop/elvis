/// line-style in `MultiColumn`
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum MultiColumnLineStyle {
    /// None style
    None,
    /// Hide the line style
    Hidden,
    /// Dotted line
    Dotted,
    /// Dashed line
    Dashed,
    /// Solid line
    Solid,
    /// Double line
    Double,
    /// Groove line
    Groove,
    /// Ridge line
    Ridge,
    /// Inset line
    Inset,
    /// OutSet line
    OutSet,
}

impl Default for MultiColumnLineStyle {
    fn default() -> MultiColumnLineStyle {
        MultiColumnLineStyle::None
    }
}

impl ToString for MultiColumnLineStyle {
    fn to_string(&self) -> String {
        match self {
            MultiColumnLineStyle::Dashed => "dashed",
            MultiColumnLineStyle::Dotted => "dotted",
            MultiColumnLineStyle::Double => "double",
            MultiColumnLineStyle::Groove => "groove",
            MultiColumnLineStyle::Hidden => "hidden",
            MultiColumnLineStyle::Inset => "inset",
            MultiColumnLineStyle::None => "none",
            MultiColumnLineStyle::OutSet => "outset",
            MultiColumnLineStyle::Ridge => "ridge",
            MultiColumnLineStyle::Solid => "solid",
        }
        .into()
    }
}
