//! Evlis styles

macro_rules! option_to_style {
    ($styles:expr, $name:ident, $field:expr) => {{
        if let Some(v) = $field {
            $styles.push(Style::$name(v));
        }
    }};
    ($styles:expr, [$(($name:ident, $field:expr),)*]) => {{
        $(option_to_style!($styles, $name, $field);)*
    }};
}

mod border;
mod r#box;
mod bridge;
mod column;
mod flex;
mod grid;
mod init;
mod typo;

pub use self::{
    border::Border, column::MultiColumnStyle, flex::FlexStyle, grid::GridStyle, init::Style,
    r#box::ContainerStyle, typo::TextStyle,
};
