use crate::Widget;
use elvis::{Life, Serde};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct State(Life);

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(widget: Widget) -> State {
        State::from(widget)
    }

    #[wasm_bindgen(js_name = "setState")]
    pub fn set_state(&mut self, k: String, v: String) {
        self.set(k, v);
    }

    #[wasm_bindgen(js_name = "getState")]
    pub fn get_state(&self) -> String {
        if self.0.state.is_empty() {
            return "{}".into();
        }

        let mut json = "".to_string();
        for (k, v) in self.0.state.iter() {
            json.push_str("\"");
            json.push_str(&k);
            json.push_str("\": \"");
            json.push_str(&v);
            json.push_str("\",");
        }

        format!("{{{}}}", &json[..(json.len() - 1)])
    }

    pub fn ser(&self) -> String {
        self.create();
        self.0.tree.ser()
    }

    pub fn create(&self) {}
    pub fn render() -> Widget {
        Widget::default()
    }
    pub fn update() {}
    pub fn dispose() {}
}

impl std::convert::From<Widget> for State {
    fn from(widget: Widget) -> State {
        State(Life::new(widget.into()))
    }
}

deref!(State, Life);
