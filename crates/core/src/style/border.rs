use crate::{
    style::Style,
    value::{BorderStyle, Color, Unit},
};
use elvis_core_support::Setter;

/// Border Style
#[derive(Default, Clone, Setter, Eq, PartialEq, Ord, PartialOrd)]
pub struct Border {
    top_color: Color,
    top_style: BorderStyle,
    top_width: Unit,

    right_color: Color,
    right_style: BorderStyle,
    right_width: Unit,

    bottom_color: Color,
    bottom_style: BorderStyle,
    bottom_width: Unit,

    left_color: Color,
    left_style: BorderStyle,
    left_width: Unit,

    top_left_radius: Unit,
    top_right_radius: Unit,
    bottom_right_radius: Unit,
    bottom_left_radius: Unit,
}

impl Border {
    /// Set color for all borders
    pub fn color(mut self, color: Color) -> Self {
        self.top_color = color;
        self.right_color = color;
        self.bottom_color = color;
        self.left_color = color;
        self
    }

    /// Set radius for all borders
    pub fn radius(mut self, radius: Unit) -> Self {
        self.top_left_radius = radius;
        self.top_right_radius = radius;
        self.bottom_right_radius = radius;
        self.bottom_left_radius = radius;
        self
    }

    /// Set width for all borders
    pub fn width(mut self, width: Unit) -> Self {
        self.top_width = width;
        self.right_width = width;
        self.bottom_width = width;
        self.left_width = width;
        self
    }

    /// Set style for all borders
    pub fn style(mut self, style: BorderStyle) -> Self {
        self.top_style = style.clone();
        self.right_style = style.clone();
        self.bottom_style = style.clone();
        self.left_style = style;
        self
    }
}

impl Into<Vec<Style>> for Border {
    fn into(self) -> Vec<Style> {
        vec![
            Style::BorderTopWidth(self.top_width),
            Style::BorderTopStyle(self.top_style),
            Style::BorderTopColor(self.top_color),
            Style::BorderRightWidth(self.right_width),
            Style::BorderRightStyle(self.right_style),
            Style::BorderRightColor(self.right_color),
            Style::BorderBottomWidth(self.bottom_width),
            Style::BorderBottomStyle(self.bottom_style),
            Style::BorderBottomColor(self.bottom_color),
            Style::BorderLeftWidth(self.left_width),
            Style::BorderLeftStyle(self.left_style),
            Style::BorderLeftColor(self.left_color),
            Style::BorderTopLeftRadius(self.top_left_radius),
            Style::BorderTopRightRadius(self.top_right_radius),
            Style::BorderBottomRightRadius(self.bottom_right_radius),
            Style::BorderBottomLeftRadius(self.bottom_right_radius),
        ]
    }
}
