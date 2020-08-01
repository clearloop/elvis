//! Evlis styles
mod basic;
mod border;
mod bridge;
mod column;
mod flex;
mod grid;
mod init;
mod typo;

pub use self::{
    basic::{ContainerStyle, SizedBoxStyle},
    border::Border,
    column::MultiColumnStyle,
    flex::FlexStyle,
    grid::GridStyle,
    init::Style,
    typo::{ImageSrc, TextStyle},
};
