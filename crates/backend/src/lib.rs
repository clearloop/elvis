//! Elvis Backend
mod cargo;
mod client;
mod err;
mod html;
mod manifest;
mod server;

#[macro_use]
extern crate log;

pub use self::{err::Error, html::HTML_TEMPLATE, manifest::Crate};
