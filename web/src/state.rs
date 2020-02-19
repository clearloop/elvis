use crate::{Func, Widget};
use elvis::State as ElvisState;
use js_sys::Function;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

/// state manager
#[wasm_bindgen]
pub struct State(ElvisState<Widget>);
deref!(State, ElvisState<Widget>);

#[wasm_bindgen]
impl State {
    pub fn new(
        widget: Widget,
        create: Function,
        update: Function,
        render: Function,
        dispose: Function,
    ) -> State {
        State(ElvisState::new(
            widget,
            Box::new(Func(create)),
            Box::new(Func(update)),
            Box::new(Func(render)),
            Box::new(Func(dispose)),
        ))
    }
}
