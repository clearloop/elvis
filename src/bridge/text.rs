use crate::{Serde, Text, Tree};
use std::{collections::HashMap, convert::Into};

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
