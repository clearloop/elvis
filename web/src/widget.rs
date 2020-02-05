use elvis::Tree;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

/// mock web element
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ElvisWidget(Tree);

impl ElvisWidget {
    pub fn new(tree: Tree) -> ElvisWidget {
        ElvisWidget(tree)
    }
}

#[wasm_bindgen]
impl ElvisWidget {
    #[wasm_bindgen(js_name = "setState")]
    pub fn set_state(&mut self, k: String, v: String) {
        self.set(&k, &v);
    }
}

deref!(ElvisWidget, Tree);
