#![macro_use]

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

macro_rules! into_node {
    {[$($sw:ident,)*], [$($mcw:ident,)*], [$($mcws:ident,)*],} => {
        // Single child widgets
        $(
            impl<'s> Into<Node> for &'s $sw {
                fn into(self) -> Node {
                    Node::new(
                        vec![Rc::new(RefCell::new(self.child.to_owned()))],
                        None,
                        "div".into(),
                    ).borrow().to_owned().class(&mut vec![Class::Flex])
                }
            }
        )*

        // Multi child widgets
        $(
            impl<'i> Into<Node> for &'i $mcw {
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

        // Multi child widgets with styles
        $(
            impl<'i> Into<Node> for &'i $mcws {
                fn into(self) -> Node {
                    let mut cs = vec![];
                    self.children.iter().for_each(|x| {
                        cs.push(Rc::new(RefCell::new(x.to_owned())));
                    });

                    Node::new(cs, None, "div".into())
                        .borrow()
                        .to_owned()
                        .style(self.style.clone())
                }
            }

        )*

        it! {
            $($sw,)*
            $($mcw,)*
            $($mcws,)*
        }
    }
}
