use crate::{layout::Center, Tree};
use std::{cell::RefCell, collections::HashMap, convert::Into, rc::Rc};

// layouts
impl<'i> Into<Tree> for &'i Center {
    fn into(self) -> Tree {
        let mut m = HashMap::<String, String>::new();
        m.insert("class".into(), "elvis-center elvis-flex".into());

        let mut cs = vec![];

        cs.push(Rc::new(RefCell::new(self.child.to_owned())));
        Tree::new(m, cs, None, "div".into()).borrow().to_owned()
    }
}
