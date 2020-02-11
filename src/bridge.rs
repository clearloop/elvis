//! Convert widgets to Tree
use crate::{Image, Serde, Text, Tree};
use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    convert::Into,
    hash::Hasher,
    rc::Rc,
};

fn hash(tag: &str, s: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(s);

    let res = format!("{:x}", hasher.finish());
    format!("{}-{}", &tag, &res[(res.len() - 6)..])
}

// widgets
impl<'t> Into<Tree> for &'t Text {
    fn into(self) -> Tree {
        let s = self.style.ser();
        let id = hash("text", s.as_bytes());
        let mut m = HashMap::<String, String>::new();
        let mut cm = HashMap::<String, String>::new();

        m.insert("id".into(), id);
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
        let id = hash("img", self.src.as_bytes());
        let mut m = HashMap::<String, String>::new();
        m.insert("class".into(), "elvis-image".into());
        m.insert("id".into(), id);
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

macro_rules! it {
    {$($widget:ident,)*} => {
        $(
            impl Into<Tree> for $widget {
                fn into(self) -> Tree {
                    let ref s = self;
                    s.into()
                }
            }
        )*
    };
}

it! {
    Image,
    Text,
}
