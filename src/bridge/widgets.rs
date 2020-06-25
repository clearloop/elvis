use crate::{
    widgets::{Image, Text},
    Serde, Tree,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl<'t> Into<Tree> for &'t Text {
    fn into(self) -> Tree {
        let mut m = HashMap::<String, String>::new();
        let mut cm = HashMap::<String, String>::new();

        m.insert("style".into(), self.style.ser());
        cm.insert("text".into(), self.text.to_string());

        Tree::new(
            m,
            vec![Tree::new(cm, vec![], None, "plain".into())],
            None,
            "p".into(),
        )
        .borrow()
        .to_owned()
    }
}

impl<'i> Into<Tree> for &'i Image {
    fn into(self) -> Tree {
        let mut m = HashMap::<String, String>::new();
        m.insert("class".into(), "elvis-image".into());
        m.insert("style".into(), self.src.ser());

        Tree::new(
            m,
            vec![Rc::new(RefCell::new(self.child.to_owned()))],
            None,
            "div".into(),
        )
        .borrow()
        .to_owned()
    }
}
