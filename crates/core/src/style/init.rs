use crate::value::{
    layouts::{
        FlexBasis, FlexDirection, FlexPosition, FlexWrap, GridAuto, GridFlow, GridTemplate,
        MultiColumnLineStyle,
    },
    BorderStyle, Color, FontFamily, FontStyle, TextAlign, Unit,
};

fn camel_snake(camel: &str) -> String {
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
    (
        [$(($ns:ident, $ty:ty, $ndoc:expr),)*],
        [$(($ss:ident, $sdoc:expr),)*]
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
    };
}

construct_style! {[
    // Box
    (Width, Unit, "Box Width"),
    (MaxWidth, Unit, "Box Max Width"),
    (Height, Unit, "Box Height"),
    (MaxHeight, Unit, "Box Max Height"),
    (Padding, Unit, "Box Padding"),
    (Margin, Unit, "Box Margin"),

    // Border
    (BorderTopWidth, Unit, "Border Top Color"),
    (BorderTopStyle, BorderStyle, "Border Top Color"),
    (BorderTopColor, Color, "Border Top Color"),

    (BorderRightWidth, Unit, "Border Right Color"),
    (BorderRightStyle, BorderStyle, "Border Right Color"),
    (BorderRightColor, Color, "Border Right Color"),

    (BorderBottomWidth, Unit, "Border Bottom Color"),
    (BorderBottomStyle, BorderStyle, "Border Bottom Color"),
    (BorderBottomColor, Color, "Border Bottom Color"),

    (BorderLeftWidth, Unit, "Border Left Color"),
    (BorderLeftStyle, BorderStyle, "Border Left Color"),
    (BorderLeftColor, Color, "Border Left Color"),

    (BorderTopLeftRadius, Unit, "Border Top Left Radius"),
    (BorderTopRightRadius, Unit, "Border Top Right Radius"),
    (BorderBottomRightRadius, Unit, "Border Bottom Right Radius"),
    (BorderBottomLeftRadius, Unit, "Border Bottom Left Radius"),

    // Typo
    (FontWeight, Unit, "Font Weight Style"),
    (FontSize, Unit, "Font Size Style"),
    (FontStretch, Unit, "Font Stretch Style"),
    (LineHeight, Unit, "Line Height Style"),

    // Color
    (Color, Color, "Custom Color"),
    (BackgroundColor, Color, "Custom Color"),

    // Flex
    (AlignItems, FlexPosition, "AlignItem Style"),
    (JustifyContent, FlexPosition, "AlignItem Style"),
    (FlexGrow, Unit, "FlexGrow Style"),
    (Order, Unit, "FlexOrder Style"),

    // Grid
    (GridAutoColumns, GridAuto, "GridAutoColumn Style"),
    (GridAutoRows, GridAuto, "GridAutoColumn Style"),
    (GridAutoFlow, GridFlow, "GridAutoFlow Style"),
    (GridColumnGap, Unit, "GridColumnGap Style"),
    (GridRowGap, Unit, "GridColumnGap Style"),
    (GridTemplateColumns, GridTemplate, "GridTemplateColumns Style"),
    (GridTemplateRows, GridTemplate, "GridTemplateRow Style"),

    // Column
    (ColumnCount, Unit, "ColumnCount Style"),
    (ColumnGap, Unit, "ColumnGap Style"),
    (ColumnRuleColor, Color, "ColumnRuleColor Style"),
    (ColumnRuleStyle, MultiColumnLineStyle, "ColumnRuleStyle Style"),
], [
    // Flex
    (FlexBasis, "FlexBasis style"),
    (FlexDirection, "FlexDirection style"),
    (FlexPosition, "FlexPosition style"),
    (FlexWrap, "Flex Wrap Style"),
    (BorderStyle, "Border Style"),

    // Grid
    (GridAuto, "Grid Auto Style"),
    (GridFlow, "Grid Flow Style"),
    (GridTemplate, "Grid Template Style"),

    // Font
    (FontStyle, "Font Style"),
    (FontFamily, "Font Family"),

    // Typo
    (TextAlign, "Text Align"),
]}
