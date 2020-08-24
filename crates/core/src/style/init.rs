use crate::value::{
    layouts::{
        FlexBasis, FlexDirection, FlexPosition, FlexWrap, GridAuto, GridFlow, GridTemplate,
        MultiColumnLineStyle,
    },
    BorderRadius, BorderStyle, BoxBorder, BoxShadow, Color, Display, FontFamily, FontStyle,
    Position, TextAlign, Unit, VecUnit,
};

pub fn camel_snake(camel: &str) -> String {
    let mut res = "".to_string();
    camel.trim().chars().enumerate().for_each(|(n, c)| {
        if n > 0 && c.is_ascii_uppercase() {
            res.push_str("-");
        }
        res.push(c);
    });

    res.to_lowercase()
}

macro_rules! construct_style {
    ($trait:ident, $fn_name:tt) => {

    };
    (
        [$(($ns:ident, $ty:tt, $nf:tt, $ndoc:expr),)*],
        [$(($ss:ident, $sf:tt, $sdoc:expr),)*]
    ) => {
        /// Evlis Style
        #[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
        pub enum Style {
            $(
                #[doc=$ndoc]
                $ns($ty),
            )*
            $(
                #[doc=$sdoc]
                $ss($ss),
            )*
        }

        impl Style {
            /// Convert `Style` to css string
            pub fn to_css(&self) -> String {
                match self {
                    $(
                        Style::$ns(v) => format!(
                            "{}: {}",
                            camel_snake(stringify!($ns)),
                            v.to_string()
                        ),
                    )*
                    $(
                        Style::$ss(v) => format!(
                            "{}: {}",
                            camel_snake(stringify!($ss)),
                            v.to_string()
                        ),
                    )*
                }
            }
        }

        $(
            impl From<$ss> for Style {
                fn from(s: $ss) -> Style {
                    Style::$ss(s)
                }
            }
        )*

        /// Style traits
        pub mod traits {
            use crate::{Style, Node};
            $(
                #[doc=$ndoc]
                pub trait $ns {
                    #[doc=$ndoc]
                    fn $nf(self, value: super::$ty) -> Node;
                }

                impl<T> $ns for T
                where
                    T: Into<Node>,
                {
                    fn $nf(self, value: super::$ty) -> Node {
                        let mut node: Node = self.into();
                        node.style.push(Style::$ns(value));
                        node
                    }
                }
            )*

            $(
                #[doc=$sdoc]
                pub trait $ss {
                    #[doc=$sdoc]
                    fn $sf(self, value: super::$ss) -> Node;
                }

                impl<T> $ss for T
                where
                    T: Into<Node>,
                {
                    fn $sf(self, value: super::$ss) -> Node {
                        let mut node: Node = self.into();
                        node.style.push(Style::$ss(value));
                        node
                    }
                }
             )*
        }
    };
}

construct_style! {[
    // Box
    (Width, Unit, width, "Box Width"),
    (Height, Unit, height, "Box Height"),
    (MaxWidth, Unit, max_width, "Box Max Width"),
    (MaxHeight, Unit, max_height, "Box Max Height"),
    (OutlineWidth, Unit, outline_width, "Box Outline Width"),
    (Top, Unit, top, "Box Top"),
    (Right, Unit, right, "Box Right"),
    (Bottom, Unit, bottom, "Box Bottom"),
    (Left, Unit, left, "Box Left"),
    (Padding, VecUnit, padding, "Box Padding"),
    (PaddingTop, Unit, padding_top, "Padding Top"),
    (PaddingRight, Unit, padding_right, "Padding right"),
    (PaddingBottom, Unit, padding_bottom, "Padding Bottom"),
    (PaddingLeft, Unit, padding_left, "Padding left"),
    (Margin, VecUnit, margin, "Box Margin"),
    (MarginTop, Unit, margin_top, "Margin Top"),
    (MarginRight, Unit, margin_right, "Margin right"),
    (MarginBottom, Unit, margin_bottom, "Margin Bottom"),
    (MarginLeft, Unit, margin_left, "Margin left"),

    // Border
    (BorderTop, BoxBorder, border_top, "Border Top"),
    (BorderRight, BoxBorder, border_right, "Border Right"),
    (BorderBottom, BoxBorder, border_bottom, "Border Bottom"),
    (BorderLeft, BoxBorder, border_left, "Border Left"),
    (Border, BoxBorder, border, "Border Border"),

    // Typo
    (FontWeight, Unit, font_weight, "Font Weight Style"),
    (FontSize, Unit, font_size, "Font Size Style"),
    (FontStretch, Unit, font_stretch, "Font Stretch Style"),
    (LineHeight, Unit, line_hegiht, "Line Height Style"),

    // Color
    (Color, Color, color, "Custom Color"),
    (BackgroundColor, Color, background_color, "Custom Color"),

    // Flex
    (AlignItems, FlexPosition, align_items, "AlignItem Style"),
    (JustifyContent, FlexPosition, justify_content, "AlignItem Style"),
    (FlexGrow, Unit, flex_glow, "FlexGrow Style"),
    (Order, Unit, order, "FlexOrder Style"),

    // Grid
    (GridAutoColumns, GridAuto, grid_auto_columns, "GridAutoColumn Style"),
    (GridAutoRows, GridAuto, grid_auto_rows, "GridAutoColumn Style"),
    (GridAutoFlow, GridFlow, grid_auto_flow, "GridAutoFlow Style"),
    (GridColumnGap, Unit, grid_column_gap, "GridColumnGap Style"),
    (GridRowGap, Unit, grid_row_rap, "GridColumnGap Style"),
    (GridTemplateColumns, GridTemplate, grid_template_columns, "GridTemplateColumns Style"),
    (GridTemplateRows, GridTemplate, grid_tempalte_row, "GridTemplateRow Style"),

    // Column
    (ColumnCount, Unit, column_count, "ColumnCount Style"),
    (ColumnGap, Unit, column_gap, "ColumnGap Style"),
    (ColumnRuleColor, Color, column_rule_color, "ColumnRuleColor Style"),
    (ColumnRuleStyle, MultiColumnLineStyle, column_rule_style, "ColumnRuleStyle Style"),
], [
    // Flex
    (FlexBasis, flex_basis, "FlexBasis style"),
    (FlexDirection, flex_direction, "FlexDirection style"),
    (FlexPosition, flex_position, "FlexPosition style"),
    (FlexWrap, flex_wrap, "Flex Wrap Style"),
    (BorderStyle, border_style, "Border Style"),

    // Grid
    (GridAuto, grid_auto, "Grid Auto Style"),
    (GridFlow, grid_flow, "Grid Flow Style"),
    (GridTemplate, grid_template, "Grid Template Style"),

    // Font
    (FontStyle, font_style, "Font Style"),
    (FontFamily, font_family, "Font Family"),

    // Typo
    (TextAlign, text_align, "Text Align"),

    // Box
    (BoxShadow, box_shadow, "Box Shadow"),
    (Position, position, "Box Position"),

    // border radius
    (BorderRadius, border_radius, "Border Radius"),
    (Display, display, "display"),
]}
