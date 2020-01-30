use elvis::{Serde, Tree};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn de_tree_empty_string() {
    let plain = "";
    assert_eq!(Tree::de(plain.into()).unwrap(), Tree::default());
}

#[test]
fn de_tree_pure_tag() {
    assert_eq!(
        Tree::de("<div></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: HashMap::new(),
            children: vec![],
        }
    );
}

#[test]
fn de_tree_has_plain_content() {
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    attrs.insert("text", "elvis");

    let pr = Rc::new(RefCell::new(Tree {
        pre: None,
        tag: "div",
        attrs: HashMap::new(),
        children: vec![],
    }));

    let pc = Rc::new(RefCell::new(Tree {
        pre: Some(Rc::downgrade(&pr)),
        tag: "plain",
        attrs: attrs,
        children: vec![],
    }));

    pr.borrow_mut().children.push(pc);

    let pr = pr.borrow().to_owned();
    assert_eq!(Tree::de("<div>elvis</div>".into()).unwrap(), pr);
    assert_eq!(Tree::de("<div>elvis</ div>".into()).unwrap(), pr);
    assert_eq!(Tree::de("<div>elvis</ div >".into()).unwrap(), pr);
    assert_eq!(Tree::de("< div>elvis</div>".into()).unwrap(), pr);
    assert_eq!(Tree::de("< div >elvis</div>".into()).unwrap(), pr);
}

#[test]
fn de_tree_tag_and_style() {
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    attrs.insert("style", "height: 20;");
    assert_eq!(
        Tree::de("<div style=\"height: 20;\"></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: attrs,
            children: vec![],
        }
    );
}

#[test]
fn de_tree_tag_and_multi_attr() {
    let mut attrs = HashMap::<&'static str, &'static str>::new();
    attrs.insert("style", "height: 20;");
    attrs.insert("name", "bowie");
    attrs.insert("bool", "false");
    assert_eq!(
        Tree::de("<div style=\"height: 20;\" name=\"bowie\" bool=\"false\"></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: attrs,
            children: vec![],
        }
    );
}

#[test]
fn de_tree_has_single_tag_child() {
    assert_eq!(
        Tree::de("<div><div></div></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: HashMap::new(),
            children: vec![Rc::new(RefCell::new(Tree {
                pre: None,
                tag: "div",
                attrs: HashMap::new(),
                children: vec![],
            }))],
        }
    );
}

#[test]
fn de_tree_has_deep_single_tag_child() {
    assert_eq!(
        Tree::de("<div><div><div></div></div></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: HashMap::new(),
            children: vec![Rc::new(RefCell::new(Tree {
                pre: None,
                tag: "div",
                attrs: HashMap::new(),
                children: vec![Rc::new(RefCell::new(Tree {
                    pre: None,
                    tag: "div",
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
        Tree::de("<div><a><b><p></p></b></a></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: HashMap::new(),
            children: vec![Rc::new(RefCell::new(Tree {
                pre: None,
                tag: "a",
                attrs: HashMap::new(),
                children: vec![Rc::new(RefCell::new(Tree {
                    pre: None,
                    tag: "b",
                    attrs: HashMap::new(),
                    children: vec![Rc::new(RefCell::new(Tree {
                        pre: None,
                        tag: "p",
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
        Tree::de("<div><div></div><div></div></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: HashMap::new(),
            children: vec![
                Rc::new(RefCell::new(Tree {
                    pre: None,
                    tag: "div",
                    attrs: HashMap::new(),
                    children: vec![]
                })),
                Rc::new(RefCell::new(Tree {
                    pre: None,
                    tag: "div",
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
        Tree::de("<div><a></a><b></b><p></p></div>".into()).unwrap(),
        Tree {
            pre: None,
            tag: "div",
            attrs: HashMap::new(),
            children: vec![
                Rc::new(RefCell::new(Tree {
                    pre: None,
                    tag: "a",
                    attrs: HashMap::new(),
                    children: vec![]
                })),
                Rc::new(RefCell::new(Tree {
                    pre: None,
                    tag: "b",
                    attrs: HashMap::new(),
                    children: vec![]
                })),
                Rc::new(RefCell::new(Tree {
                    pre: None,
                    tag: "p",
                    attrs: HashMap::new(),
                    children: vec![]
                }))
            ],
        }
    );
}
