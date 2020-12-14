//! Gestrue Trait
use crate::{Closure, Node, StateKV};
use std::{collections::HashMap, sync::Arc};

/// Gestures
/// Construct gestures
macro_rules! construct_gesture {
    ($(($name:tt, $doc:expr),)*) => {
        /// Elvis Gestures
        #[derive(Debug, Hash, PartialEq, Eq, Clone)]
        pub enum Gesture {
            $(
                #[doc=$doc]
                $name,
            )*
        }
    };
}

construct_gesture! {
    (Tap, "Trigger when tap widget"),
    (LongTap, "Trigger when long tap widget"),
}

/// Gesture HashMap
pub type GestureKV = HashMap<Gesture, Closure<StateKV>>;

/// Gestrue Detector
#[derive(Clone)]
pub struct GestureDetector<W> {
    child: W,
    gesture: GestureKV,
}

impl<W> GestureDetector<W>
where
    W: Into<Node>,
{
    /// New Gesture Detector
    pub fn new(n: W) -> GestureDetector<W> {
        GestureDetector {
            child: n,
            gesture: HashMap::new(),
        }
    }

    /// Register method
    pub fn register(mut self, gesture: Gesture, callback: fn(StateKV) -> ()) -> Self {
        self.gesture
            .entry(gesture)
            .or_insert_with(|| Arc::new(callback));

        self
    }

    /// Get method
    pub fn get(&mut self, name: Gesture) -> Option<&Closure<StateKV>> {
        if let Some(f) = self.gesture.get(&Box::new(name)) {
            Some(f)
        } else {
            None
        }
    }

    /// Remove and return method
    pub fn remove(&mut self, name: Gesture) -> Option<Closure<StateKV>> {
        self.gesture.remove(&Box::new(name))
    }

    /// List methods and closures
    pub fn list(&self) -> Vec<(&Gesture, &Closure<StateKV>)> {
        self.gesture
            .iter()
            .map(|(m, c)| (m, c))
            .collect::<Vec<(&Gesture, &Closure<StateKV>)>>()
    }
}

impl<W> Into<Node> for GestureDetector<W>
where
    W: Into<Node>,
{
    fn into(self) -> Node {
        let mut n = self.child.into();
        n.gesture = Some(self.gesture);
        n.state = Some(HashMap::new());
        n
    }
}
