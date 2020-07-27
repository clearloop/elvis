use crate::{widgets::layouts::Center, Node};
use elvis_core::Class;
use std::{cell::RefCell, collections::HashMap, convert::Into, rc::Rc};

// layouts
impl<'i> Into<Node> for &'i Center {
    fn into(self) -> Node {
        let mut cs = vec![];
        cs.push(Rc::new(RefCell::new(self.child.to_owned())));

        Node::new(HashMap::new(), cs, None, "div".into())
            .borrow_mut()
            .to_owned()
            .append_class(vec![Class::Flex, Class::Center])
    }
}
