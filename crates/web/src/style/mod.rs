//! Web style

mod parser;
mod sheet;

pub use self::{
    parser::{parse_class, parse_style},
    sheet::StyleSheet,
};
