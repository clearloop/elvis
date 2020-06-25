use elvis::widgets::{layouts::Center, Text, TextStyle};
use elvis_web::Widget;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    let mut center = Widget::new(Center {
        child: Text {
            text: "hello, world".into(),
            style: TextStyle::default(),
        }
        .into(),
    });

    center.calling().unwrap();
}
