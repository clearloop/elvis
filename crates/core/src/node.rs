use crate::{Class, GestureKV, StateKV, Style};
use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    fmt,
    hash::Hasher,
    rc::{Rc, Weak},
};

fn hash(tag: &str, s: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(s);

    let res = format!("{:x}", hasher.finish());
    format!("{}-{}", &tag, &res[(res.len() - 6)..])
}

/// Virtual UI Node
#[derive(Clone, Default)]
pub struct Node {
    /// Node attributes
    pub attrs: HashMap<String, String>,
    /// Node Class
    pub class: Vec<Class>,
    /// Node Class
    pub style: Vec<Style>,
    /// Node children
    pub children: Vec<Rc<RefCell<Node>>>,
    /// Node tag
    pub tag: String,
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
            .field("attrs", &self.attrs)
            .field("children", &self.children)
            .field("tag", &self.tag)
            .field("pre", &self.pre)
            .finish()
    }
}

impl Node {
    /// Append class
    pub fn append_class(mut self, classes: Vec<Class>) -> Node {
        self.class = classes;
        self.class.sort();
        self.class.dedup();
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
        let h = hash(&self.tag, &path);
        self.attrs.entry("id".into()).or_insert(h);

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

    /// Generate a `Rc<RefCell<Node>>`
    pub fn new(
        attrs: HashMap<String, String>,
        children: Vec<Rc<RefCell<Node>>>,
        pre: Option<Weak<RefCell<Node>>>,
        tag: String,
    ) -> Rc<RefCell<Node>> {
        let t = Node {
            attrs,
            children,
            pre,
            tag,
            class: vec![],
            style: vec![],
            state: None,
            gesture: None,
        };

        Rc::new(RefCell::new(t))
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
        let res = self.attrs.eq(&other.attrs) && self.tag.eq(&other.tag);

        for (p, q) in self.children.iter().enumerate() {
            if !q.eq(&other.children[p]) {
                return false;
            }
        }

        res
    }
}
