use crate::Error;
use std::collections::HashMap;

/// store closures
pub trait FnBox<P> {
    /// Call function
    fn call(&mut self, props: &P) -> Result<(), Error>;
}

/// Func hook
type Hook<P> = Box<dyn FnBox<P>>;

/// state for tree
pub struct State<W, P> {
    /// Host widget
    pub widget: W,
    /// Function Hook
    pub trigger: Hook<P>,
    state: HashMap<String, String>,
}

impl<W, P> State<W, P> {
    /// New state
    pub fn new(widget: W, trigger: Hook<P>) -> State<W, P> {
        State {
            state: HashMap::new(),
            widget,
            trigger,
        }
    }

    /// Trigger function
    pub fn process(&mut self, p: &P) -> Result<(), Error> {
        self.trigger.call(p)
    }

    /// Get state
    pub fn get(&self, k: String) -> String {
        self.state.get(&k).unwrap_or(&"".to_string()).to_string()
    }

    /// Set state
    pub fn set(&mut self, k: &str, v: &str) {
        self.state.insert(k.to_string(), v.to_string());
    }
}
