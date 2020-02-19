use crate::{Func, StyleSheet, Widget};
use elvis::{Serde, State as ElvisState};
use js_sys::Function;
use std::ops::{Deref, DerefMut};
use std::{cell::RefCell, collections::HashSet, rc::Rc};
use wasm_bindgen::prelude::*;

/// state manager
#[wasm_bindgen]
pub struct State(Rc<RefCell<ElvisState<Widget>>>);
deref!(State, Rc<RefCell<ElvisState<Widget>>>);

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(widget: Widget, create: Function, update: Function, dispose: Function) -> State {
        State(Rc::new(RefCell::new(ElvisState::new(
            widget,
            Box::new(Func(create)),
            Box::new(Func(update)),
            Box::new(Func(dispose)),
        ))))
    }

    pub fn set_state(&mut self, k: String, v: String) {
        self.0.borrow_mut().set(&k, &v)
    }
}

impl State {
    pub fn ser(&self) -> String {
        self.borrow().widget.ser()
    }

    pub fn style(&mut self) -> String {
        self.0.borrow_mut().create().unwrap_or(());
        StyleSheet::batch(&mut self.0.borrow_mut().widget, &mut HashSet::new())
            .trim()
            .to_string()
    }
}
