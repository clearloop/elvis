#![feature(extern_types)]
#[macro_use]
mod macros;
mod color;
mod elvis;
mod style;
mod types;
mod widget;

pub use crate::{
    color::Colors,
    elvis::Elvis,
    style::TextStyle,
    widget::{text, Widget},
};
