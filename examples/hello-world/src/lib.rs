use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    let mut center = elvis_web::center(elvis_web::text(
        Some("hello, world".into()),
        None,
    ));

    center.calling().unwrap();
}
