use crate::Error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Virtual DOM Tree
#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub attrs: HashMap<&'static str, &'static str>,
    pub children: Vec<Rc<RefCell<Tree>>>,
    pub pre: Option<Weak<RefCell<Tree>>>,
    pub tag: &'static str,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        match (&self.pre, &other.pre) {
            (Some(s), Some(o)) => s.ptr_eq(o),
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(feature = "web")]
pub mod web;
