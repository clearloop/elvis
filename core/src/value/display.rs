/// Display
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Display {
    /// Block Display
    Block,
    /// InlineBlock Display
    InlineBlock,
    /// Flex Display
    Flex,
    /// Grid Display
    Grid,
}

impl ToString for Display {
    fn to_string(&self) -> String {
        match self {
            Display::Block => "block",
            Display::InlineBlock => "inline-block",
            Display::Flex => "flex",
            Display::Grid => "grid",
        }
        .into()
    }
}
