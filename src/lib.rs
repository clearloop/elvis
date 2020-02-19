//! # RoadMap
//!
//! | Plan              | Date                  |
//! |-------------------|-----------------------|
//! | Virtual-DOM       | 2020.1.28 - 2020.2.1  |
//! | Basic-Component   | 2020.2.5 - 2020.2.7   |
//! | Router            | 2020.2.7 - 2020.2.10  |
//! | Native Components | 2020.2.10 - 2020.2.11 |
//! | Benchmark         | 2020.2.11 - 2020.2.15 |
//! | Beta Versin       | 2020.2.15 - 2020.2.17 |
//!
//! # Calling Elvis
//!
//! Is anybody home?
//!
//! As we know, `Elvis` is a famous rock star, and both a famous rock song named `Calling Elvis` wrote by `Dire Straits` which inspired a unknown rock star to write down these chords(code || words).
//!
//! For now, Elvis, the rock star, will rise, once again, **beyond** the internet —— **truely your wasm web library**.
//!
//! [The Evlis Book][1] mainly talks about the usage of the npm package `calling-elvis`, and if you want to use `"low-level"` api rusting the web, plz check out [elvis' rust doc][2].
//!
//! ## Goals
//!
//! Writing web pages in **pure javascript** using wasm bindings, **without** `jsx` or `any other` complex syntax, **just javascript**, of course, **not** writing `html` nor `css` either.
//!
//! ## Roll up for the Magical Mystery Tour!
//!
//! Here we go! Roll up, roll up for the mystery tour, the magical mystery tour is waiting to take you away! Hoping to take you away! Coming to take you away! Dying to take you away, take you today!
//!
//! Let me take you down, cause I'm going to,
//!
//! ```js
//! /* javascript
//!
//! const Home = Text("Pink is the Pig!", {
//!   bold: true,
//!   italic: true,
//!   size: 10,
//!   color: Colors.PinkAccent(),
//! });
//!
//! new Elvis({
//!   home: Home,
//! }).calling();
//! ```
//!  **Strawberry Fields**.
//!
//!
//! ## LICENSE
//!
//! Heartbreak Hotel.
//!
//! [1]: https://clearloop.github.io/elvis
//! [2]: https://docs.rs/elvis
#![feature(try_trait)]
#![feature(box_syntax)]
#![feature(vec_remove_item)]
mod bridge;
mod err;
mod func;
mod layout;
mod serde;
mod state;
mod tree;
mod values;
mod widgets;

pub use crate::{
    err::Error,
    func::FnBox,
    layout::*,
    serde::Serde,
    state::State,
    tree::Tree,
    values::{color::Colors, layout::*, unit::Unit},
    widgets::{Image, ImageSrc, Text, TextStyle},
};

// elvis platform features
/// Browser data bridge, as default feature.
#[cfg(feature = "web")]
mod web;
