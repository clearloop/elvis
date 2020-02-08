use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// Virtual UI Tree
#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub attrs: HashMap<String, String>,
    pub children: Vec<Rc<RefCell<Tree>>>,
    pub tag: String,
    pub pre: Option<Weak<RefCell<Tree>>>,
}

impl Tree {
    /// drain tree if not the root
    pub fn drain(t: Rc<RefCell<Tree>>) {
        if let Some(pre) = &t.clone().borrow().pre {
            let u = pre.upgrade().expect("drain child failed");
            u.borrow_mut().remove(t);

            u.borrow().update();
        }
    }

    pub fn locate(&self, mut path: Vec<usize>) -> Vec<usize> {
        if let Some(pre) = &self.pre {
            let u = pre.upgrade().expect("locate widget failed");
            for (i, t) in u.borrow().children.iter().enumerate() {
                if t.borrow().eq(self) {
                    path.push(i);
                    return u.borrow().locate(path);
                }
            }
        }

        path
    }

    /// generate a Rc<RefCell<Tree>>
    pub fn new(
        attrs: HashMap<String, String>,
        children: Vec<Rc<RefCell<Tree>>>,
        pre: Option<Weak<RefCell<Tree>>>,
        tag: String,
    ) -> Rc<RefCell<Tree>> {
        let t = Tree {
            attrs,
            children,
            pre,
            tag,
        };

        Rc::new(RefCell::new(t))
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

        r.borrow().update();
    }

    /// delete spefic child using rc
    pub fn remove(&mut self, c: Rc<RefCell<Tree>>) {
        self.children.remove_item(&c);
        self.update();
    }

    /// replace current tree
    pub fn replace(&mut self, mut t: Tree) {
        t.pre = self.pre.clone();
        std::mem::swap(self, &mut t);

        t.update();
    }

    pub fn update(&self) {}
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
