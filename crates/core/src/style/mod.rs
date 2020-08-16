//! Evlis styles
/// Optional fields to style
#[macro_export]
macro_rules! option_to_style {
    ($styles:expr, $name:ident, $field:expr) => {{
        if let Some(v) = $field {
            $styles.push(Style::$name(v));
        }
    }};
    ($styles:expr, $field:expr) => {{
        if let Some(v) = $field {
            $styles.append(&mut v.into());
        }
    }};
    (
        $styles:expr,
        [$(($name:ident, $field:expr),)*],
    ) => {{
        $(option_to_style!($styles, $name, $field);)*
    }};
    (
        $styles:expr,
        [$(($name:ident, $field:expr),)*],
        [$($sfield:expr,)*],
    ) => {{
        $(option_to_style!($styles, $name, $field);)*
        $(option_to_style!($styles, $sfield);)*
    }};
}

mod border;
mod bridge;
mod column;
mod flex;
mod grid;
mod init;

pub use self::{
    border::Border,
    column::MultiColumnStyle,
    flex::FlexStyle,
    grid::GridStyle,
    init::{traits, Style},
};
