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

    /// add second tree to the first one.
    pub fn push(r: Rc<RefCell<Tree<'t>>>, c: Rc<RefCell<Tree<'t>>>) {
        let pre = Rc::downgrade(&r);
        c.borrow_mut().pre = Some(pre.clone());

        pre.upgrade()
            .expect("push child to tree failed")
            .borrow_mut()
            .children
            .push(c);
    }

    /// drain tree if not the root
    pub fn drain(t: Rc<RefCell<Tree<'t>>>) {
        if let Some(pre) = &t.clone().borrow().pre {
            let u = pre.upgrade().expect("drain child failed");
            u.borrow_mut().remove(t);
        }
    }

    /// delete spefic child using rc
    pub fn remove(&mut self, c: Rc<RefCell<Tree<'t>>>) {
        self.children.remove_item(&c);
    }

    /// replace current tree
    pub fn replace(&mut self, mut t: Tree<'t>) {
        t.pre = self.pre.clone();
        std::mem::swap(self, &mut t);
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
