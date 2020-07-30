use crate::widgets::{Image, Text};
use elvis_core::Node;
use std::{cell::RefCell, rc::Rc};

impl<'t> Into<Node> for &'t Text {
    fn into(self) -> Node {
        let child = Node::new(vec![], None, "plain".into());
        child.borrow_mut().attr.text = self.text.to_string();

        Node::new(vec![child], None, "p".into())
            .borrow()
            .to_owned()
            .style(self.style)
    }
}

impl<'i> Into<Node> for &'i Image {
    fn into(self) -> Node {
        Node::new(
            vec![Rc::new(RefCell::new(self.child.to_owned()))],
            None,
            "div".into(),
        )
        .borrow()
        .to_owned()
    }
}
