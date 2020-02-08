//! LifeCycle in Elvis
use crate::Tree;
use std::collections::HashMap;

/// `Life` manager
pub struct Life {
    pub tree: Tree,
    pub state: HashMap<String, String>,
}

impl Life {
    pub fn new(tree: Tree) -> Life {
        Life {
            tree,
            state: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: String, v: String) {
        self.state.insert(k, v);
    }
}

/// Lifecycle
///
/// 1. `create()` calling when constructs
/// 2. `update()` calling after `set_state()`
/// 3. `render()` calling after `create()` and `update()`
/// 4. `dispose()` calling after deleting tree
pub trait LifeCycle<T> {
    fn create(&mut self);
    fn update(&self);
    fn render() -> T;
    fn dispose(&mut self);
}
