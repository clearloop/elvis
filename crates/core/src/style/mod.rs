//! Evlis styles

/// Evlis Style
#[derive(Clone)]
pub enum Style {
    /// Box Height
    Height,
    /// Box Width
    Width,
}

impl ToString for Style {
    fn to_string(&self) -> String {
        match self {
            Style::Height => "height",
            Style::Width => "width",
        }
        .into()
    }
}

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
