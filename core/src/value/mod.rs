//! Elvis values
mod border;
mod r#box;
mod color;
mod column;
mod display;
mod flex;
mod font;
mod grid;
mod typo;
mod unit;

/// Elvis layout values
pub mod layouts {
    pub use super::column::MultiColumnLineStyle;
    pub use super::flex::{Alignment, FlexBasis, FlexDirection, FlexPosition, FlexWrap};
    pub use super::grid::{GridAuto, GridFlow, GridTemplate};
}

pub use {
    border::{BorderRadius, BorderStyle, BoxBorder},
    color::Color,
    display::Display,
    font::{FontFamily, FontStyle},
    r#box::{BoxShadow, Position},
    typo::TextAlign,
    unit::{Unit, VecUnit},
};
