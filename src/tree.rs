use crate::LifeCycle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Virtual UI Tree
#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub attrs: HashMap<String, String>,
    pub state: HashMap<String, String>,
    pub children: Vec<Rc<RefCell<Tree>>>,
    pub tag: String,
    pub pre: Option<Weak<RefCell<Tree>>>,
}

impl Tree {
    /// generate a Rc<RefCell<Tree>>
    pub fn new(
        attrs: HashMap<String, String>,
        state: HashMap<String, String>,
        children: Vec<Rc<RefCell<Tree>>>,
        pre: Option<Weak<RefCell<Tree>>>,
        tag: String,
    ) -> Rc<RefCell<Tree>> {
        // gen lifecycle create method
        <Self as LifeCycle<Self>>::create();

        // new tree
        Rc::new(RefCell::new(Tree {
            attrs,
            state,
            children,
            pre,
            tag,
        }))
    }

    /// add second tree to the first one.
    pub fn push(r: Rc<RefCell<Tree>>, c: Rc<RefCell<Tree>>) {
        let pre = Rc::downgrade(&r);
        c.borrow_mut().pre = Some(pre.clone());

        pre.upgrade()
            .expect("push child to tree failed")
            .borrow_mut()
            .children
            .push(c);

        <Self as LifeCycle<Self>>::update();
    }

    /// drain tree if not the root
    pub fn drain(t: Rc<RefCell<Tree>>) {
        if let Some(pre) = &t.clone().borrow().pre {
            let u = pre.upgrade().expect("drain child failed");
            u.borrow_mut().remove(t);

            <Self as LifeCycle<Self>>::update();
        }
    }

    /// delete spefic child using rc
    pub fn remove(&mut self, c: Rc<RefCell<Tree>>) {
        self.children.remove_item(&c);
        <Self as LifeCycle<Self>>::update();
    }

    /// replace current tree
    pub fn replace(&mut self, mut t: Tree) {
        t.pre = self.pre.clone();
        std::mem::swap(self, &mut t);

        <Self as LifeCycle<Self>>::update();
    }

    /// set state
    pub fn set<'t>(&mut self, k: &'t str, v: &'t str) {
        self.state.insert(k.into(), v.into());

        <Self as LifeCycle<Self>>::update();
    }
}

impl<'t> Drop for Tree {
    fn drop(&mut self) {
        <Self as LifeCycle<Self>>::dispose();
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        let res = self.attrs.eq(&other.attrs) && self.tag.eq(&other.tag);

        for (p, q) in self.children.iter().enumerate() {
            if !q.eq(&other.children[p]) {
                return false;
            }
        }

        res
    }
}
