use elvis::Tree;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
/// basic widget without lifecycle nor state
#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Widget(Tree);

impl Widget {
    /// new widget from tree
    pub fn new(tree: Tree) -> Widget {
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

impl std::convert::Into<Tree> for Widget {
    fn into(self) -> Tree {
        self.0
    }
}

deref!(Widget, Tree);
