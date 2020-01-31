use crate::{Serde, Text, Tree};
use std::{
    collections::HashMap,
    convert::{From, Into},
};

impl<'t> Into<Tree<'t>> for Text {
    fn into(self) -> Tree<'t> {
        let mut m = HashMap::<&'t str, String>::new();
        let mut cm = HashMap::<&'t str, String>::new();

        m.insert("style", self.style.ser());
        cm.insert("text", self.text.into());

        Tree::new(m, vec![Tree::new(cm, vec![], None, "plain")], None, "p")
            .borrow()
            .to_owned()
    }
}
