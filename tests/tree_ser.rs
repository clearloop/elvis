use elvis_shared::{Node, Serde};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn ser_tree_pure_tag() {
    let t = Node {
        pre: None,
        tag: "div".into(),
        attrs: HashMap::new(),
        children: vec![],
        state: None,
        gesture: None,
    };

    assert_eq!(t.ser(), "<div></div>".to_string());
}

#[test]
fn ser_tree_attrs_tag() {
    let mut m = HashMap::<String, String>::new();
    m.insert("name".into(), "elvis".into());
    let t = Node {
        pre: None,
        tag: "div".into(),
        attrs: m.clone(),
        children: vec![],
        state: None,
        gesture: None,
    };

    assert_eq!(t.ser(), "<div name=\"elvis\"></div>".to_string());
}

#[test]
fn ser_tree_plain_content() {
    let mut m = HashMap::<String, String>::new();
    m.insert("text".into(), "hello, world!".into());
    let t = Node {
        pre: None,
        tag: "div".into(),
        attrs: HashMap::new(),
        children: vec![Rc::new(RefCell::new(Node {
            pre: None,
            tag: "plain".into(),
            attrs: m,
            children: vec![],
            state: None,
            gesture: None,
        }))],
        state: None,
        gesture: None,
    };

    assert_eq!(t.ser(), "<div>hello, world!</div>".to_string());
}

#[test]
fn ser_tree_inner_tag() {
    let m = HashMap::<String, String>::new();
    let t = Node {
        pre: None,
        tag: "div".into(),
        attrs: m.clone(),
        children: vec![Rc::new(RefCell::new(Node {
            pre: None,
            tag: "div".into(),
            attrs: m,
            children: vec![],
            state: None,
            gesture: None,
        }))],
        state: None,
        gesture: None,
    };

    assert_eq!(t.ser(), "<div><div></div></div>".to_string());
}
