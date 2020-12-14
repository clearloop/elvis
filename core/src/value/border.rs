use crate::value::{Color, Unit};
use elvis_core_support::{EnumStyle, Setter};

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

/// Border Style
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Setter)]
pub struct BoxBorder {
    /// border width
    pub width: Unit,
    /// border style
    pub style: BorderStyle,
    /// border color
    pub color: Color,
}

impl BoxBorder {
    /// The shortcut of `BoxBorder::new()`
    pub fn with(width: Unit) -> BoxBorder {
        BoxBorder::new().width(width)
    }
}

impl Default for BoxBorder {
    fn default() -> BoxBorder {
        BoxBorder {
            width: Unit::None(0.0),
            style: BorderStyle::None,
            color: Color::Black,
        }
    }
}

impl ToString for BoxBorder {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.width.to_string(),
            self.style.to_string(),
            self.color.to_string()
        )
    }
}

/// Border Radius
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Setter)]
pub struct BorderRadius {
    /// top left radius
    pub top_left: Unit,
    /// top right radius
    pub top_right: Unit,
    /// bottom left radius
    pub bottom_left: Unit,
    /// bottom right radius
    pub bottom_right: Unit,
    /// second top left radius
    pub second_top_left: Unit,
    /// second top right radius
    pub second_top_right: Unit,
    /// second bottom left radius
    pub second_bottom_left: Unit,
    /// second bottom right radius
    pub second_bottom_right: Unit,
}

impl BorderRadius {
    /// Set all radius
    pub fn all(self, radius: Unit) -> BorderRadius {
        self.top_left(radius)
            .top_right(radius)
            .bottom_left(radius)
            .bottom_right(radius)
    }
}

impl Default for BorderRadius {
    fn default() -> BorderRadius {
        BorderRadius {
            top_left: Unit::None(0.0),
            top_right: Unit::None(0.0),
            bottom_left: Unit::None(0.0),
            bottom_right: Unit::None(0.0),
            second_top_left: Unit::None(0.0),
            second_top_right: Unit::None(0.0),
            second_bottom_left: Unit::None(0.0),
            second_bottom_right: Unit::None(0.0),
        }
    }
}

impl ToString for BorderRadius {
    fn to_string(&self) -> String {
        let radius = format!(
            "{} {} {} {}",
            self.top_left.to_string(),
            self.top_right.to_string(),
            self.bottom_left.to_string(),
            self.bottom_right.to_string(),
        );

        if self.second_top_left == self.second_top_right
            && self.second_bottom_right == self.second_bottom_left
            && self.second_top_left == self.second_bottom_right
            && self.second_bottom_right == Unit::None(0.0)
        {
            radius
        } else {
            format!(
                "{} / {} {} {} {}",
                radius,
                self.second_top_left.to_string(),
                self.second_top_right.to_string(),
                self.second_bottom_left.to_string(),
                self.second_bottom_right.to_string(),
            )
        }
    }
}
