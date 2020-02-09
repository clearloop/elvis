use crate::{Image, Tree};
use std::{cell::RefCell, rc::Rc};
use std::{collections::HashMap, convert::Into};

impl<'i> Into<Tree> for &'i Image {
    fn into(self) -> Tree {
        let mut m = HashMap::<String, String>::new();
        m.insert("src".into(), self.src.to_string());
        m.insert("style".into(), "height: 100%; width: 100%;".into());

        Tree::new(
            m,
            vec![Rc::new(RefCell::new(self.child.to_owned()))],
            None,
            "p".into(),
        )
        .borrow()
        .to_owned()
    }
}
