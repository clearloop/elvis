use crate::widgets::layouts::Center;
use elvis_core::{Class, Node};
use std::{cell::RefCell, convert::Into, rc::Rc};

// layouts
impl<'i> Into<Node> for &'i Center {
    fn into(self) -> Node {
        let mut cs = vec![];
        cs.push(Rc::new(RefCell::new(self.child.to_owned())));

        Node::new(cs, None, "div".into())
            .borrow_mut()
            .to_owned()
            .class(&mut vec![Class::Flex, Class::Center])
    }
}
