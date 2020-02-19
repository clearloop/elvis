use crate::{Error, FnBox};
use std::collections::HashMap;

type Hook = Box<dyn FnBox>;

/// state for tree
pub struct State<W> {
    pub widget: W,
    state: HashMap<String, String>,
    create: Hook,
    update: Hook,
    render: Hook,
    dispose: Hook,
}

impl<W> State<W> {
    pub fn new(widget: W, create: Hook, update: Hook, render: Hook, dispose: Hook) -> State<W> {
        State {
            state: HashMap::new(),
            widget,
            create,
            update,
            render,
            dispose,
        }
    }

    pub fn create(self) -> Result<(), Error> {
        self.create.call_box()
    }

    pub fn update(self) -> Result<(), Error> {
        self.update.call_box()
    }

    pub fn render(self) -> Result<(), Error> {
        self.render.call_box()
    }

    pub fn dispose(self) -> Result<(), Error> {
        self.dispose.call_box()
    }

    pub fn get(&self, k: String) -> String {
        self.state.get(&k).unwrap_or(&"".to_string()).to_string()
    }

    pub fn set<'s>(&mut self, k: &str, v: &str) {
        self.state.insert(k.to_string(), v.to_string());
    }
}