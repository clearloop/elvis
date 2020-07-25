#![macro_use]

/// multi-child widget
macro_rules! mcw {
    {$($widget:ident,)*} => {
        $(
            impl<'i> Into<Node> for &'i $widget {
                fn into(self) -> Node {
                    let mut m = HashMap::<String, String>::new();
                    if stringify!($widget) != "List" {
                        m.insert(
                            "class".into(),
                            format!("elvis-{} elvis-flex", stringify!($widget).to_lowercase())
                        );
                    }

                    let mut cs = vec![];
                    self.children.iter().for_each(|x| {
                        cs.push(Rc::new(RefCell::new(x.to_owned())));
                    });

                    Node::new(m, cs, None, "div".into())
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
            impl<'i> Into<Node> for &'i $widget {
                fn into(self) -> Node {
                    let ss = self.style.to_string();
                    let mut m = HashMap::<String, String>::new();
                    m.insert("style".into(), ss);

                    let mut cs = vec![];
                    self.children.iter().for_each(|x| {
                        cs.push(Rc::new(RefCell::new(x.to_owned())));
                    });

                    Node::new(m, cs, None, "div".into())
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
            impl Into<Node> for $widget {
                fn into(self) -> Node {
                    let s = &self;
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
            impl<'s> Into<Node> for &'s $widget {
                fn into(self) -> Node {
                    let ss = self.style.to_string();
                    let mut m = HashMap::<String, String>::new();
                    m.insert("style".into(), ss);
                    m.insert("class".into(), "elvis-flex".into());

                    Node::new(
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
