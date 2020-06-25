use crate::{
    widgets::{Image, Text},
    Node, Serde,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl<'t> Into<Node> for &'t Text {
    fn into(self) -> Node {
        let mut m = HashMap::<String, String>::new();
        let mut cm = HashMap::<String, String>::new();

        m.insert("style".into(), self.style.ser());
        cm.insert("text".into(), self.text.to_string());

        Node::new(
            m,
            vec![Node::new(cm, vec![], None, "plain".into())],
            None,
            "p".into(),
        )
        .borrow()
        .to_owned()
    }
}

impl<'i> Into<Node> for &'i Image {
    fn into(self) -> Node {
        let mut m = HashMap::<String, String>::new();
        m.insert("class".into(), "elvis-image".into());
        m.insert("style".into(), self.src.ser());

        Node::new(
            m,
            vec![Rc::new(RefCell::new(self.child.to_owned()))],
            None,
            "div".into(),
        )
        .borrow()
        .to_owned()
    }
}
