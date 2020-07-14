//! Gesture Detector
use crate::{Closure, Node};
use std::{collections::HashMap, hash::Hash};

/// Gestrue Trait
///
/// This is a bridge for elvis gesture widgets and pure UI widgets
pub trait Gesture<N, P> {
    /// Gesture Name
    fn name(&self) -> N;
    /// Gesture callback function
    fn callback(&self) -> Box<Closure<P>>;
}

/// Gestrue Detector
pub struct GestureDetector<M, P> {
    map: HashMap<Box<M>, Box<Closure<P>>>,
    /// Elvis Node
    pub child: Node,
}

impl<N, P> GestureDetector<N, P>
where
    N: Eq + Hash + Clone,
{
    /// New Gesture Detector
    pub fn new(child: Node) -> GestureDetector<N, P> {
        GestureDetector {
            map: HashMap::new(),
            child,
        }
    }

    /// Register method
    pub fn register<G>(&mut self, gesture: G)
    where
        G: Gesture<N, P> + Sized,
    {
        self.map
            .entry(Box::new(gesture.name()))
            .or_insert_with(|| gesture.callback());
    }

    /// Get method
    pub fn get(&mut self, name: N) -> Option<&Closure<P>> {
        if let Some(f) = self.map.get(&Box::new(name)) {
            Some(f.as_ref())
        } else {
            None
        }
    }

    /// Remove and return method
    pub fn remove(&mut self, name: N) -> Option<Box<Closure<P>>> {
        self.map.remove(&Box::new(name))
    }

    /// List methods and closures
    pub fn list(&self) -> Vec<(&N, &Closure<P>)> {
        self.map
            .iter()
            .map(|(m, c)| (m.as_ref(), c.as_ref()))
            .collect::<Vec<(&N, &Closure<P>)>>()
    }
}
