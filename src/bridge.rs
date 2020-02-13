//! Convert widgets to Tree
use crate::{
    Align, Center, Col, Container, Flex, Grid, Image, List, MultiColumn, Row, Serde, SizedBox,
    Text, Tree,
};
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

// layouts
impl<'i> Into<Tree> for &'i Center {
    fn into(self) -> Tree {
        let mut m = HashMap::<String, String>::new();
        m.insert("class".into(), "elvis-center".into());

        let mut cs = vec![];
        cs.push(Rc::new(RefCell::new(self.child.to_owned())));
        Tree::new(m, cs, None, "div".into()).borrow().to_owned()
    }
}

/// multi-child widget
macro_rules! mcw {
    {$($widget:ident,)*} => {
        $(
            impl<'i> Into<Tree> for &'i $widget {
                fn into(self) -> Tree {
                    let mut m = HashMap::<String, String>::new();
                    m.insert(
                        "class".into(),
                        format!("elvis-{} elvis-flex", stringify!($widget).to_lowercase())
                    );

                    let mut cs = vec![];
                    self.children.iter().for_each(|x| {
                        cs.push(Rc::new(RefCell::new(x.to_owned())));
                    });

                    Tree::new(m, cs, None, "div".into())
                        .borrow()
                        .to_owned()
                }
            }
        )*
    }
}

/// multi-child widget with style
macro_rules! mcws {
    {$($widget:ident,)*} => {
        $(
            impl<'i> Into<Tree> for &'i $widget {
                fn into(self) -> Tree {
                    let ss = self.style.ser();
                    let id = hash(&stringify!($widget).to_lowercase(), ss.as_bytes());
                    let mut m = HashMap::<String, String>::new();
                    m.insert("id".into(), id);
                    m.insert("style".into(), ss);
                    m.insert("class".into(), "elvis-flex".into());

                    let mut cs = vec![];
                    self.children.iter().for_each(|x| {
                        cs.push(Rc::new(RefCell::new(x.to_owned())));
                    });

                    Tree::new(m, cs, None, "div".into())
                        .borrow()
                        .to_owned()
                }
            }

            it! {
                $widget,
            }
        )*
    }
}

/// owned widget into tree
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

/// single child widgets
macro_rules! sw {
    {$($widget:ident,)*} => {
        $(
            impl<'s> Into<Tree> for &'s $widget {
                fn into(self) -> Tree {
                    let ss = self.style.ser();
                    let id = hash(&stringify!($widget).to_lowercase(), ss.as_bytes());
                    let mut m = HashMap::<String, String>::new();
                    m.insert("id".into(), id);
                    m.insert("style".into(), ss);
                    m.insert("class".into(), "elvis-flex".into());

                    Tree::new(
                        m,
                        vec![Rc::new(RefCell::new(self.child.to_owned()))],
                        None,
                        "div".into(),
                    ).borrow().to_owned()
                }
            }

            it! {
                $widget,
            }
        )*
    };
}

sw! {
    Align,
    Container,
    Flex,
    SizedBox,
}

mcw! {
    Col,
    List,
    Row,
}

mcws! {
    Grid,
    MultiColumn,
}

it! {
    Center,
    Col,
    Row,
    Image,
    Text,
    List,
}
