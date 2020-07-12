//! State machine
use std::collections::HashMap;

/// state for tree
pub struct State<W> {
    /// Host widget
    pub widget: W,
    // Function Hook
    // pub trigger: Hook<P>,
    state: HashMap<String, String>,
}

impl<W> State<W> {
    /// New state
    pub fn new(widget: W) -> State<W> {
        State {
            state: HashMap::new(),
            widget,
        }
    }

    // Trigger function
    // pub fn process(&mut self, p: &P) -> Result<(), Error> {
    //     self.trigger.call(p)
    // }

    /// Get state
    pub fn get(&self, k: String) -> String {
        self.state.get(&k).unwrap_or(&"".to_string()).to_string()
    }

    /// Set state
    pub fn set(&mut self, k: &str, v: &str) {
        self.state.insert(k.to_string(), v.to_string());
    }
}
