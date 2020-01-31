use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Virtual UI Tree
#[derive(Clone, Debug, Default)]
pub struct Tree<'t> {
    pub attrs: HashMap<&'t str, String>,
    pub children: Vec<Rc<RefCell<Tree<'t>>>>,
    pub pre: Option<Weak<RefCell<Tree<'t>>>>,
    pub tag: &'t str,
}

impl<'t> Tree<'t> {
    /// generate a Rc<RefCell<Tree>>
    pub fn new(
        attrs: HashMap<&'t str, String>,
        children: Vec<Rc<RefCell<Tree<'t>>>>,
        pre: Option<Weak<RefCell<Tree<'t>>>>,
        tag: &'t str,
    ) -> Rc<RefCell<Tree<'t>>> {
        Rc::new(RefCell::new(Tree {
            attrs,
            children,
            pre,
            tag,
        }))
    }

    /// appends two tree, use after `Tree::de` usually.
    pub fn append(r: Self, mut c: Self) -> Rc<RefCell<Tree<'t>>> {
        let rt = Rc::new(RefCell::new(r));
        c.pre = Some(Rc::downgrade(&rt));
        rt.borrow_mut().children.push(Rc::new(RefCell::new(c)));

        rt
    }
}

impl PartialEq for Tree<'_> {
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
