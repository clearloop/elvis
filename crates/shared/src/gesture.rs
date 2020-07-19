//! Gestrue Trait
use crate::{Closure, Node, StateKV};
use std::collections::HashMap;

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
    (Tap, "hello"),
    (LongTap, "hello"),
}

/// Gesture HashMap
pub type GestureKV = HashMap<Gesture, Closure<StateKV>>;

/// Gestrue Detector
#[derive(Clone)]
pub struct GestureDetector<W> {
    child: W,
    gestures: GestureKV,
}

impl<W> GestureDetector<W>
where
    W: Into<Node>,
{
    /// New Gesture Detector
    pub fn new(n: W) -> GestureDetector<W> {
        GestureDetector {
            child: n,
            gestures: HashMap::new(),
        }
    }

    /// Register method
    pub fn register(&mut self, gesture: Gesture, callback: Closure<HashMap<Vec<u8>, Vec<u8>>>) {
        self.gestures.entry(gesture).or_insert_with(|| callback);
    }

    /// Get method
    pub fn get(&mut self, name: Gesture) -> Option<&Closure<StateKV>> {
        if let Some(f) = self.gestures.get(&Box::new(name)) {
            Some(f)
        } else {
            None
        }
    }

    /// Remove and return method
    pub fn remove(&mut self, name: Gesture) -> Option<Closure<StateKV>> {
        self.gestures.remove(&Box::new(name))
    }

    /// List methods and closures
    pub fn list(&self) -> Vec<(&Gesture, &Closure<StateKV>)> {
        self.gestures
            .iter()
            .map(|(m, c)| (m, c))
            .collect::<Vec<(&Gesture, &Closure<StateKV>)>>()
    }
}
