//! Init command
use std::env;
use super::new;
use crate::template::APP_TEMPLATE;

/// Init Elvis APP
pub fn run() {
    match env::current_dir() {
        Ok(p) => new::run(p, APP_TEMPLATE),
        Err(e) => error!("Exec epm init failed: {:?}", e),
    }
}
