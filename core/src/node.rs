use crate::{Attribute, Class, GestureKV, StateKV, Style, StyleWrapper};
use elvis_core_support::Wrapper;
use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    fmt,
    hash::Hasher,
    rc::{Rc, Weak},
};

fn hash(s: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(s);

    let res = format!("{:x}", hasher.finish());
    format!("elvis-{}", &res[0..6])
}

/// Virtual UI Node
#[derive(Clone, Default, Wrapper)]
pub struct Node {
    /// Node attribute
    pub attr: Attribute,
    /// Node Class
    pub class: Vec<Class>,
    /// Node Class
    pub style: Vec<Style>,
    /// Node children
    pub children: Vec<Rc<RefCell<Node>>>,
    /// Node parent
    pub pre: Option<Weak<RefCell<Node>>>,
    /// Node state
    pub state: Option<StateKV>,
    /// Node Gestures
    pub gesture: Option<GestureKV>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("children", &self.children)
            .field("pre", &self.pre)
            .finish()
    }
}

impl Node {
    /// Set attribute
    pub fn attr(mut self, attr: Attribute) -> Node {
        self.attr = attr;
        self
    }

    /// Set Children
    pub fn children(mut self, children: Vec<Node>) -> Node {
        self.children = children
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect::<Vec<Rc<RefCell<Node>>>>();
        self
    }

    /// append child
    pub fn append_child(mut self, child: Node) -> Node {
        self.children.push(Rc::new(RefCell::new(child)));
        self
    }

    /// append children
    pub fn append_children(mut self, children: Vec<Node>) -> Node {
        self.children.append(
            &mut children
                .iter()
                .map(|n| Rc::new(RefCell::new(n.clone())))
                .collect::<Vec<Rc<RefCell<Node>>>>(),
        );
        self
    }

    /// Set class
    pub fn class(mut self, class: Vec<Class>) -> Node {
        self.class = class;
        self
    }

    /// Append class
    pub fn append_class(mut self, class: &mut Vec<Class>) -> Node {
        self.class.append(class);
        self.class.sort();
        self.class.dedup();
        self
    }

    /// Set style
    pub fn style(mut self, style: impl Into<Vec<Style>>) -> Node {
        self.style = style.into();
        self
    }

    /// Append style
    pub fn append_style(mut self, styles: impl Into<Vec<Style>>) -> Node {
        self.style.append(&mut styles.into());
        self.style.sort();
        self.style.dedup();
        self
    }

    /// Drain tree if not the root
    pub fn drain(t: Rc<RefCell<Node>>) {
        if let Some(pre) = &t.borrow().pre {
            let u = pre.upgrade().expect("drain child failed");
            u.borrow_mut().remove(t.clone());
            u.borrow_mut().update();
        }
    }

    /// The path of current node
    pub fn idx(&mut self, path: &mut Vec<u8>) {
        self.attr.id = hash(&path);

        path.push(0);
        for t in self.children.iter() {
            t.borrow_mut().idx(path);
            if let Some(last) = path.last_mut() {
                *last += 1;
            }
        }
    }

    /// Locate tree
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

    /// Add second tree to the first one.
    pub fn push(r: Rc<RefCell<Node>>, c: Rc<RefCell<Node>>) {
        let pre = Rc::downgrade(&r);
        c.borrow_mut().pre = Some(pre.clone());

        pre.upgrade()
            .expect("push child to tree failed")
            .borrow_mut()
            .children
            .push(c);

        r.borrow_mut().update();
    }

    /// Delete spefic child using rc
    pub fn remove(&mut self, c: Rc<RefCell<Node>>) {
        self.children.retain(|x| x != &c);
        self.update();
    }

    /// Replace current tree
    pub fn replace(&mut self, mut t: Node) {
        t.pre = self.pre.clone();
        std::mem::swap(self, &mut t);

        t.update();
    }

    /// Update tree
    pub fn update(&mut self) {}
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        let res =
            self.attr.eq(&other.attr) && self.style.eq(&other.style) && self.class.eq(&other.class);

        for (p, q) in self.children.iter().enumerate() {
            if !q.eq(&other.children[p]) {
                return false;
            }
        }

        res
    }
}

impl Eq for Node {}
