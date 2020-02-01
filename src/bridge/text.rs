use crate::{Serde, Text, Tree};
use std::{collections::HashMap, convert::Into};

impl<'t> Into<Tree<'t>> for Text {
    fn into(self) -> Tree<'t> {
        let mut m = HashMap::<&'t str, String>::new();
        let mut cm = HashMap::<&'t str, String>::new();

        m.insert("style", self.style.ser());
        cm.insert("text", self.text.into());

        Tree::new(
            m,
            HashMap::new(),
            vec![Tree::new(cm, HashMap::new(), vec![], None, "plain")],
            None,
            "p",
        )
        .borrow()
        .to_owned()
    }
}
