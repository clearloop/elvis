//! State machine
use crate::Node;
use std::collections::HashMap;

/// State store map
pub type StateKV = HashMap<Vec<u8>, Vec<u8>>;

/// state for tree
pub struct State<W> {
    /// Elvis Node
    pub child: W,
    /// State Machine
    pub state: StateKV,
}

impl<W> State<W>
where
    W: Into<Node>,
{
    /// Get state
    pub fn get(&self, k: &[u8]) -> Vec<u8> {
        self.state.get(k).unwrap_or(&vec![]).to_vec()
    }

    /// Set state
    pub fn set(&mut self, k: &[u8], v: &[u8]) {
        self.state.insert(k.to_vec(), v.to_vec());
    }
}

impl<W> Into<Node> for State<W>
where
    W: Into<Node>,
{
    fn into(self) -> Node {
        let mut n = self.child.into();
        n.state = Some(self.state);
        n
    }
}
