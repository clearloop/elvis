use super::{Color, Unit};
use elvis_core_support::EnumStyle;
use std::cmp::Ordering;

/// Box Shadow
#[derive(Clone, PartialEq, Eq)]
pub enum BoxShadow {
    /// None Style
    None,
    /// Inherit Style
    Inherit,
    /// Initial Style
    Initial,
    /// Inset Style
    Inset,
    /// Unset Style
    Unset,
    /// Unit
    Unit(Unit),
    /// Color
    Color(Color),
    /// offset-x | offset-y | blur-radius | spread-radius | color
    Customize(Vec<BoxShadow>),
    /// Derive BoxShadows
    Derive(Vec<BoxShadow>),
}

impl Default for BoxShadow {
    fn default() -> BoxShadow {
        BoxShadow::None
    }
}

impl PartialOrd for BoxShadow {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        self.to_string().partial_cmp(&o.to_string())
    }
}

impl Ord for BoxShadow {
    fn cmp(&self, o: &Self) -> Ordering {
        self.to_string().cmp(&o.to_string())
    }
}

impl ToString for BoxShadow {
    fn to_string(&self) -> String {
        match self {
            BoxShadow::None => "none".to_string(),
            BoxShadow::Inset => "inset".to_string(),
            BoxShadow::Inherit => "inherit".to_string(),
            BoxShadow::Initial => "initial".to_string(),
            BoxShadow::Unset => "unset".to_string(),
            BoxShadow::Unit(v) => v.to_string(),
            BoxShadow::Color(v) => v.to_string(),
            BoxShadow::Customize(v) => v
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            BoxShadow::Derive(v) => v
                .iter()
                .map(|bs| bs.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
}

/// Box Position
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, EnumStyle)]
pub enum Position {
    /// Absolute position
    Absolute,
    /// Relative position
    Relative,
}

impl Default for Position {
    fn default() -> Position {
        Position::Relative
    }
}
