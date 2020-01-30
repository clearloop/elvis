use crate::err::Error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Virtual UI Tree
#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub attrs: HashMap<&'static str, &'static str>,
    pub children: Vec<Rc<RefCell<Tree>>>,
    pub pre: Option<Weak<RefCell<Tree>>>,
    pub tag: &'static str,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        let res = self.attrs.eq(&other.attrs) && self.tag.eq(other.tag);

        for (p, q) in self.children.iter().enumerate() {
            if !q.eq(&other.children[p]) {
                return false;
            }
        }

        res
    }
}

impl Tree {
    /// generate a Rc<RefCell<Tree>>
    pub fn new(
        attrs: HashMap<&'static str, &'static str>,
        children: Vec<Rc<RefCell<Tree>>>,
        pre: Option<Weak<RefCell<Tree>>>,
        tag: &'static str,
    ) -> Rc<RefCell<Tree>> {
        Rc::new(RefCell::new(Tree {
            attrs,
            children,
            pre,
            tag,
        }))
    }

    /// appends two tree, use after `Tree::de` usually.
    pub fn append(r: Self, mut c: Self) -> Rc<RefCell<Tree>> {
        let rt = Rc::new(RefCell::new(r));
        c.pre = Some(Rc::downgrade(&rt));
        rt.borrow_mut().children.push(Rc::new(RefCell::new(c)));

        rt
    }
}

// features condition
#[allow(unused_imports)]
use crate::features::web::tree;
