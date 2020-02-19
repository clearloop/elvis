use crate::{Error, FnBox};
use std::collections::HashMap;

type Hook = Box<dyn FnBox>;

/// state for tree
pub struct State<W> {
    pub widget: W,
    state: HashMap<String, String>,
    create: Hook,
    update: Hook,
    dispose: Hook,
}

impl<W> State<W> {
    pub fn new(widget: W, create: Hook, update: Hook, dispose: Hook) -> State<W> {
        State {
            state: HashMap::new(),
            widget,
            create,
            update,
            dispose,
        }
    }

    pub fn set_widget(&mut self, w: W) {
        self.widget = w;
    }

    pub fn create(&mut self) -> Result<(), Error> {
        self.create.call()
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.update.call()
    }

    pub fn dispose(&mut self) -> Result<(), Error> {
        self.dispose.call()
    }

    pub fn get(&self, k: String) -> String {
        self.state.get(&k).unwrap_or(&"".to_string()).to_string()
    }

    pub fn set<'s>(&mut self, k: &str, v: &str) {
        self.state.insert(k.to_string(), v.to_string());
    }
}
