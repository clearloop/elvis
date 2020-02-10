//! Convert widgets to Tree
use crate::{Image, Serde, Text, Tree};
use std::{cell::RefCell, collections::HashMap, convert::Into, rc::Rc};

/// Text
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
        m.insert(
            "style".into(),
            format!(
                "{}",
                vec![
                    "height: 100%;width: 100%;",
                    &format!("background-image: url({});", &self.src),
                    "background-size: cover;",
                    "background-repeat: no-repeat;",
                    "background-position: center;"
                ]
                .join("")
            ),
        );

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
