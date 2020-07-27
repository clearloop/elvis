use crate::{
    widgets::{Image, Text},
    Node,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl<'t> Into<Node> for &'t Text {
    fn into(self) -> Node {
        let mut cm = HashMap::<String, String>::new();
        cm.insert("text".into(), (&self.text).to_string());

        Node::new(
            HashMap::new(),
            vec![Node::new(cm, vec![], None, "plain".into())],
            None,
            "p".into(),
        )
        .borrow()
        .to_owned()
        .append_style(self.style)
    }
}

impl<'i> Into<Node> for &'i Image {
    fn into(self) -> Node {
        Node::new(
            HashMap::new(),
            vec![Rc::new(RefCell::new(self.child.to_owned()))],
            None,
            "div".into(),
        )
        .borrow()
        .to_owned()
    }
}
