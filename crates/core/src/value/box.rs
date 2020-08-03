use super::{Color, Unit};
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
    /// Unset Style
    Unset,
    /// offset-x | offset-y | blur-radius | spread-radius | color
    Customize(Unit, Unit, Unit, Unit, Color),
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
            BoxShadow::Inherit => "inherit".to_string(),
            BoxShadow::Initial => "initial".to_string(),
            BoxShadow::Unset => "unset".to_string(),
            BoxShadow::Customize(x, y, blur, spread, color) => format!(
                "{} {} {} {} {}",
                x.to_string(),
                y.to_string(),
                blur.to_string(),
                spread.to_string(),
                color.to_string()
            ),
            BoxShadow::Derive(v) => v
                .iter()
                .map(|bs| bs.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
}
