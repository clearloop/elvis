#![macro_use]

/// multi-child widget
macro_rules! mcw {
    {$($widget:ident,)*} => {
        $(
            impl<'i> Into<Node> for &'i $widget {
                fn into(self) -> Node {
                    let mut cs = vec![];
                    self.children.iter().for_each(|x| {
                        cs.push(Rc::new(RefCell::new(x.to_owned())));
                    });

                    Node::new(cs, None, "div".into())
                        .borrow()
                        .to_owned()
                        .class(&mut vec![
                            Class::Flex,
                            Class::from(stringify!($widget).to_lowercase().as_str()),
                        ])
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
                    let mut cs = vec![];
                    self.children.iter().for_each(|x| {
                        cs.push(Rc::new(RefCell::new(x.to_owned())));
                    });

                    Node::new(cs, None, "div".into())
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
                    Node::new(
                        vec![Rc::new(RefCell::new(self.child.to_owned()))],
                        None,
                        "div".into(),
                    ).borrow().to_owned().class(&mut vec![Class::Flex])
                }
            }

            it! {
                $widget,
            }
        )*
    };
}
