use elvis_core_support::EnumStyle;

#[derive(EnumStyle)]
pub enum Position {
    /// Absolute position
    Absolute,
    /// Relative position
    Relative,
}
