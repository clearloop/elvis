use elvis::Node;
use elvis_shared::Serde;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn de_tree_empty_string() {
    let plain = "";
    assert_eq!(Node::de(plain.into()).unwrap(), Node::default());
}

#[test]
fn de_tree_pure_tag() {
    assert_eq!(
        Node::de("<div></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: HashMap::new(),
            children: vec![],
        }
    );
}

#[test]
fn de_tree_has_plain_content() {
    let mut attrs = HashMap::<String, String>::new();
    attrs.insert("text".into(), "elvis".into());

    let pr = Rc::new(RefCell::new(Node {
        pre: None,
        tag: "div".into(),
        attrs: HashMap::new(),
        children: vec![],
    }));

    let pc = Rc::new(RefCell::new(Node {
        pre: Some(Rc::downgrade(&pr)),
        tag: "plain".into(),
        attrs: attrs,
        children: vec![],
    }));

    pr.borrow_mut().children.push(pc);

    let pr = pr.borrow().to_owned();
    assert_eq!(Node::de("<div>elvis</div>".into()).unwrap(), pr);
    assert_eq!(Node::de("<div>elvis</ div>".into()).unwrap(), pr);
    assert_eq!(Node::de("<div>elvis</ div >".into()).unwrap(), pr);
    assert_eq!(Node::de("< div>elvis</div>".into()).unwrap(), pr);
    assert_eq!(Node::de("< div >elvis</div>".into()).unwrap(), pr);
}

#[test]
fn de_tree_tag_and_style() {
    let mut attrs = HashMap::<String, String>::new();
    attrs.insert("style".into(), "height: 20;".to_string());
    assert_eq!(
        Node::de("<div style=\"height: 20;\"></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: attrs,
            children: vec![],
        }
    );
}

#[test]
fn de_tree_tag_and_multi_attr() {
    let mut attrs = HashMap::<String, String>::new();
    attrs.insert("style".into(), "height: 20;".into());
    attrs.insert("name".into(), "bowie".into());
    attrs.insert("bool".into(), "false".into());
    assert_eq!(
        Node::de("<div style=\"height: 20;\" name=\"bowie\" bool=\"false\"></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: attrs,
            children: vec![],
        }
    );
}

#[test]
fn de_tree_has_single_tag_child() {
    assert_eq!(
        Node::de("<div><div></div></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: HashMap::new(),
            children: vec![Rc::new(RefCell::new(Node {
                pre: None,
                tag: "div".into(),
                attrs: HashMap::new(),
                children: vec![],
            }))],
        }
    );
}

#[test]
fn de_tree_has_deep_single_tag_child() {
    assert_eq!(
        Node::de("<div><div><div></div></div></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: HashMap::new(),
            children: vec![Rc::new(RefCell::new(Node {
                pre: None,
                tag: "div".into(),
                attrs: HashMap::new(),
                children: vec![Rc::new(RefCell::new(Node {
                    pre: None,
                    tag: "div".into(),
                    attrs: HashMap::new(),
                    children: vec![],
                }))],
            }))],
        }
    );
}

#[test]
fn de_tree_has_deep_multi_tag_child() {
    assert_eq!(
        Node::de("<div><a><b><p></p></b></a></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: HashMap::new(),
            children: vec![Rc::new(RefCell::new(Node {
                pre: None,
                tag: "a".into(),
                attrs: HashMap::new(),
                children: vec![Rc::new(RefCell::new(Node {
                    pre: None,
                    tag: "b".into(),
                    attrs: HashMap::new(),
                    children: vec![Rc::new(RefCell::new(Node {
                        pre: None,
                        tag: "p".into(),
                        attrs: HashMap::new(),
                        children: vec![],
                    }))],
                }))],
            }))],
        }
    );
}

#[test]
fn de_tree_has_parallel_tag_children() {
    assert_eq!(
        Node::de("<div><div></div><div></div></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: HashMap::new(),
            children: vec![
                Rc::new(RefCell::new(Node {
                    pre: None,
                    tag: "div".into(),
                    attrs: HashMap::new(),
                    children: vec![]
                })),
                Rc::new(RefCell::new(Node {
                    pre: None,
                    tag: "div".into(),
                    attrs: HashMap::new(),
                    children: vec![]
                }))
            ],
        }
    );
}

#[test]
fn de_tree_has_parallel_multi_tag_children() {
    assert_eq!(
        Node::de("<div><a></a><b></b><p></p></div>".into()).unwrap(),
        Node {
            pre: None,
            tag: "div".into(),
            attrs: HashMap::new(),
            children: vec![
                Rc::new(RefCell::new(Node {
                    pre: None,
                    tag: "a".into(),
                    attrs: HashMap::new(),
                    children: vec![]
                })),
                Rc::new(RefCell::new(Node {
                    pre: None,
                    tag: "b".into(),
                    attrs: HashMap::new(),
                    children: vec![]
                })),
                Rc::new(RefCell::new(Node {
                    pre: None,
                    tag: "p".into(),
                    attrs: HashMap::new(),
                    children: vec![]
                }))
            ],
        }
    );
}
