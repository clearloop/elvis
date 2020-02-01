use elvis::Tree;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

/// mock web element
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Widget(Tree);

impl Widget {
    pub fn new(tree: Tree) -> Widget {
        Widget(tree)
    }
}

deref!(Widget, Tree);
