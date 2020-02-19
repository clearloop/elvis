//! This library implements Elvis in browser, most of the contents are interfaces.
//!
//! You can rust the web with this crate, and...no javascript usages in this doc mainly because rust-doc can not highlight `javascript` code, we strongly recommend you reading [The Book][1] if you wondering how to "calling elvis".
//!
//! [1]: https://clearloop.github.io/elvis
#![feature(extern_types)]
#[macro_use]
mod macros;
mod app;
mod color;
mod enums;
mod func;
mod layout;
mod state;
mod style;
mod types;
mod widgets;

pub use crate::{
    app::Elvis,
    color::Colors,
    enums::{
        Alignments, FlexBasis, FlexDirection, GridAuto, GridFlow, GridTemplate,
        MultiColumnLineStyle,
    },
    func::Func,
    layout::{align, center, container, flex, sized_box, Col, Grid, List, MultiColumn, Row},
    style::{
        AlignStyle, ContainerStyle, FlexStyle, GridStyle, MultiColumnStyle, SizedBoxStyle,
        StyleSheet, TextStyle,
    },
    widgets::{img, text, Widget},
};
