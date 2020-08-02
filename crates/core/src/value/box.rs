use super::{Color, Unit};

/// Box Shadow
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
