use elvis::{LifeCycle, Tree};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
/// basic widget without lifecycle nor state
#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Widget(Tree);

impl Widget {
    /// new widget from tree
    pub fn new(mut tree: Tree) -> Widget {
        tree.create();
        Widget(tree)
    }
}

#[wasm_bindgen]
impl Widget {
    #[wasm_bindgen(constructor)]
    pub fn constructor() -> Widget {
        Widget::default()
    }
}

deref!(Widget, Tree);
