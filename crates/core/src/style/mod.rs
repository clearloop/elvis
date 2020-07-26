//! Evlis styles
use crate::value::{
    layouts::{
        FlexBasis, FlexDirection, FlexPosition, GridAuto, GridFlow, GridTemplate,
        MultiColumnLineStyle,
    },
    Colors, Unit,
};

fn camel_snake(camel: &str) -> String {
    camel
        .split(|c: char| c.is_ascii_uppercase())
        .collect::<Vec<&str>>()
        .join("_")
}

macro_rules! construct_style {
    ($(($style:ident, $ty:ty, $doc:expr),)*) => {
        /// Evlis Style
        #[derive(Clone)]
        pub enum Style {
            $(
                #[doc=$doc]
                $style($ty),
            )*
        }
    };
    (
        [$(($ns:ident, $nty:ty, $ndoc:expr),)*],
        [$(($ss:ident, $sty:ty, $sdoc:expr, $fmt:expr),)*]
    ) => {
        impl ToString for Style {
            fn to_string(&self) -> String {
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
                            camel_snake(stringify!($fmt)),
                            v.to_string()
                        ),
                    )*
                }
            }
        }
    };
    (
        [$(($ns:ident, $nty:ty, $ndoc:expr),)*],
        [$(($ds:ident, $dty:ty, $ddoc:expr),)*],
        [$(($ss:ident, $sty:ty, $sdoc:expr, $fmt:expr),)*]

    ) => {
        construct_style!{
            $(($ns, $nty, $ndoc),)*
            $(($ds, $dty, $ddoc),)*
            $(($ss, $sty, $sdoc),)*
        }
        construct_style!{[
            $(($ns, $nty, $ndoc),)*
            $(($ds, $dty, $ddoc),)*
        ], [
            $(($ss, $sty, $sdoc, $fmt),)*
        ]}

        $(
            impl From<$dty> for Style {
                fn from(s: $dty) -> Style {
                    Style::$ds(s)
                }
            }
        )*
    }
}

construct_style! {[
    // Box
    (Height, Unit, "Box Height"),
    (Width, Unit, "Box Width"),
    (Padding, Unit, "Box Padding"),
    (Margin, Unit, "Box Margin"),

    // Color
    (Color, Colors, "Custom Color"),
    (BackgroundColor, Colors, "Custom Color"),

    // Flex
    (AlignItems, FlexPosition, "AlignItem Style"),
    (JustifyContent, FlexPosition, "AlignItem Style"),
], [
    // Flex
    (FlexBasis, FlexBasis, "FlexBasis style"),
    (FlexDirection, FlexDirection, "FlexDirection style"),
    (FlexPosition, FlexPosition, "FlexPosition style"),

    // Grid
    (GridAuto, GridAuto, "Grid Auto Style"),
    (GridFlow, GridFlow, "Grid Flow Style"),
    (GridTemplate, GridTemplate, "Grid Template Style"),
], [
    (MultiColumnLine, MultiColumnLineStyle, "MultiColumnLine Style", "style"),
]}

mod basic;
mod column;
mod flex;
mod grid;
mod widget;

pub use self::{
    basic::{ContainerStyle, SizedBoxStyle},
    column::MultiColumnStyle,
    flex::{AlignStyle, FlexStyle},
    grid::GridStyle,
    widget::{ImageSrc, TextStyle},
};
