//! State machine
use crate::Node;
use std::collections::HashMap;

/// State store map
pub type StateKV = HashMap<Vec<u8>, Vec<u8>>;

/// state for tree
pub struct State {
    /// Elvis Node
    child: Node,
    /// State Machine
    state: StateKV,
}

impl State {
    /// New State
    pub fn new(node: impl Into<Node>) -> State {
        State {
            child: node.into(),
            state: HashMap::new(),
        }
    }
}

impl State {
    /// Get state
    pub fn get(&self, k: &[u8]) -> Vec<u8> {
        self.state.get(k).unwrap_or(&vec![]).to_vec()
    }

    /// Set state
    pub fn set(&mut self, k: &[u8], v: &[u8]) {
        self.state.insert(k.to_vec(), v.to_vec());
    }
}

impl Into<Node> for State {
    fn into(self) -> Node {
        let mut n: Node = self.child.into();
        n.state = Some(self.state);
        n
    }
}
