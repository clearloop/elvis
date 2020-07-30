//! WebDriver Implementation
use elvis_core::Driver as DriverTrait;
use wasm_bindgen::prelude::*;

/// Elvis web driver
pub struct Driver;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

impl DriverTrait for Driver {
    fn alert(msg: &str) {
        alert(&msg);
    }

    fn log(msg: &str) {
        log(&msg);
    }
}
