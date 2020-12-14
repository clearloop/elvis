//! Event Listener
//!
//! The source code is migrated from gloo
//!
//! [gloo]: https://github.com/rustwasm/gloo/tree/master/crates/events

mod listener;
mod option;
mod phase;

pub use self::{listener::EventListener, option::EventListenerOptions, phase::EventListenerPhase};
