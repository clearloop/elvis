//! Elvis values
mod border;
mod r#box;
mod color;
mod column;
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
    border::BorderStyle,
    color::Color,
    font::{FontFamily, FontStyle},
    r#box::BoxShadow,
    typo::TextAlign,
    unit::{Unit, VecUnit},
};
