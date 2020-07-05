//! Elvis Backend
#![warn(missing_docs)]
mod logger;

mod cargo;
mod client;
mod err;
mod html;
mod manifest;
mod server;

#[macro_use]
extern crate log;

pub use self::{err::Error, manifest::Crate};
