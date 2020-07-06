use crate::{widgets::layouts::Center, Node};
use std::{cell::RefCell, collections::HashMap, convert::Into, rc::Rc};

// layouts
impl<'i, T> Into<Node> for &'i Center<T>
where
    T: Into<Node> + Sized + Clone,
{
    fn into(self) -> Node {
        let mut m = HashMap::<String, String>::new();
        m.insert("class".into(), "elvis-center elvis-flex".into());

        let mut cs = vec![];

        cs.push(Rc::new(RefCell::new(self.child.to_owned().into())));
        Node::new(m, cs, None, "div".into()).borrow().to_owned()
    }
}
