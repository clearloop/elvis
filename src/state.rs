use crate::{Error, FnBox};
use std::collections::HashMap;

type Hook<P> = Box<dyn FnBox<P>>;

/// state for tree
pub struct State<W, P> {
    pub widget: W,
    pub trigger: Hook<P>,
    state: HashMap<String, String>,
}

impl<W, P> State<W, P> {
    pub fn new(widget: W, trigger: Hook<P>) -> State<W, P> {
        State {
            state: HashMap::new(),
            widget,
            trigger,
        }
    }

    pub fn process(&mut self, p: &P) -> Result<(), Error> {
        self.trigger.call(p)
    }

    pub fn get(&self, k: String) -> String {
        self.state.get(&k).unwrap_or(&"".to_string()).to_string()
    }

    pub fn set<'s>(&mut self, k: &str, v: &str) {
        self.state.insert(k.to_string(), v.to_string());
    }
}
