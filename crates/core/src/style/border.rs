use crate::{
    style::Style,
    value::{BorderRadius, BorderStyle, BoxBorder, Color, Unit},
};
use elvis_core_support::Setter;

/// Border Style
#[derive(Clone, Setter, Eq, PartialEq, Ord, PartialOrd)]
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

    second_top_left_radius: Unit,
    second_top_right_radius: Unit,
    second_bottom_right_radius: Unit,
    second_bottom_left_radius: Unit,
}

impl Default for Border {
    fn default() -> Border {
        Border {
            top_color: Color::Black,
            top_style: BorderStyle::default(),
            top_width: Unit::Px(1.0),

            right_color: Color::Black,
            right_style: BorderStyle::default(),
            right_width: Unit::Px(1.0),

            bottom_color: Color::Black,
            bottom_style: BorderStyle::default(),
            bottom_width: Unit::Px(1.0),

            left_color: Color::Black,
            left_style: BorderStyle::default(),
            left_width: Unit::Px(1.0),

            top_left_radius: Unit::None(0.0),
            top_right_radius: Unit::None(0.0),
            bottom_right_radius: Unit::None(0.0),
            bottom_left_radius: Unit::None(0.0),

            second_top_left_radius: Unit::None(0.0),
            second_top_right_radius: Unit::None(0.0),
            second_bottom_right_radius: Unit::None(0.0),
            second_bottom_left_radius: Unit::None(0.0),
        }
    }
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
        let top_border = BoxBorder {
            width: self.top_width,
            style: self.top_style,
            color: self.top_color,
        };
        let right_border = BoxBorder {
            width: self.right_width,
            style: self.right_style,
            color: self.right_color,
        };
        let bottom_border = BoxBorder {
            width: self.bottom_width,
            style: self.bottom_style,
            color: self.bottom_color,
        };
        let left_border = BoxBorder {
            width: self.left_width,
            style: self.left_style,
            color: self.left_color,
        };

        let mut styles = vec![Style::BorderRadius(BorderRadius {
            top_left: self.top_left_radius,
            top_right: self.top_right_radius,
            bottom_right: self.bottom_right_radius,
            bottom_left: self.bottom_left_radius,
            second_top_left: self.second_top_left_radius,
            second_top_right: self.second_top_right_radius,
            second_bottom_right: self.second_bottom_left_radius,
            second_bottom_left: self.second_bottom_right_radius,
        })];

        if top_border == right_border
            && right_border == bottom_border
            && bottom_border == left_border
        {
            styles.append(&mut vec![
                Style::BorderTop(top_border),
                Style::BorderRight(right_border),
                Style::BorderBottom(bottom_border),
                Style::BorderLeft(left_border),
            ]);
        } else {
            styles.append(&mut vec![Style::Border(top_border)]);
        }

        styles
    }
}
