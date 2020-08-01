use crate::value::{
    layouts::{
        FlexBasis, FlexDirection, FlexPosition, GridAuto, GridFlow, GridTemplate,
        MultiColumnLineStyle,
    },
    BorderStyle, Color, FontFamily, FontStyle, Unit,
};

macro_rules! construct_style {
    (
        [$(($style:ident, $ty:ty, $doc:expr),)*],
        [$(($ss:ident, $sdoc:expr),)*]
    ) => {
        /// Evlis Style
        #[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
        pub enum Style {
            $(
                #[doc=$doc]
                $style($ty),
            )*
            $(
                #[doc=$sdoc]
                $ss($ss),
            )*
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
    (Height, Unit, "Box Height"),
    (Width, Unit, "Box Width"),
    (Padding, Unit, "Box Padding"),
    (Margin, Unit, "Box Margin"),

    // Border
    (BorderTopRadius, Unit, "Border Top Color"),
    (BorderTopWidth, Unit, "Border Top Color"),
    (BorderTopStyle, BorderStyle, "Border Top Color"),
    (BorderTopColor, Color, "Border Top Color"),

    (BorderRightRadius, Unit, "Border Right Color"),
    (BorderRightWidth, Unit, "Border Right Color"),
    (BorderRightStyle, BorderStyle, "Border Right Color"),
    (BorderRightColor, Color, "Border Right Color"),

    (BorderBottomRadius, Unit, "Border Bottom Color"),
    (BorderBottomWidth, Unit, "Border Bottom Color"),
    (BorderBottomStyle, BorderStyle, "Border Bottom Color"),
    (BorderBottomColor, Color, "Border Bottom Color"),

    (BorderLeftRadius, Unit, "Border Left Color"),
    (BorderLeftWidth, Unit, "Border Left Color"),
    (BorderLeftStyle, BorderStyle, "Border Left Color"),
    (BorderLeftColor, Color, "Border Left Color"),

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
    (FlexOrder, Unit, "FlexOrder Style"),
    (Wrap, bool, "Flex Wrap"),

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
    (BorderStyle, "Border Style"),

    // Grid
    (GridAuto, "Grid Auto Style"),
    (GridFlow, "Grid Flow Style"),

    // Font
    (GridTemplate, "Grid Template Style"),
    (FontStyle, "Font Style"),
    (FontFamily, "Font Family"),
]}
