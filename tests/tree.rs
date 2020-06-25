use elvis::{Node, Serde};
use std::collections::HashMap;

#[test]
fn test_push() {
    let t = Node::de("<h1><p></p></h1>".into()).unwrap();

    let r = Node::new(HashMap::new(), vec![], None, "h1".into());
    let c = Node::new(HashMap::new(), vec![], None, "p".into());
    Node::push(r.clone(), c);
    assert_eq!(t, r.borrow().to_owned());
}

#[test]
fn test_remove() {
    let t = Node::de("<h1></h1>".into()).unwrap();

    let r = Node::new(HashMap::new(), vec![], None, "h1".into());
    let c = Node::new(HashMap::new(), vec![], None, "p".into());
    Node::push(r.clone(), c.clone());

    r.borrow_mut().remove(c);
    assert_eq!(t, r.borrow().to_owned());
}

#[test]
fn test_drain() {
    let t = Node::de("<h1></h1>".into()).unwrap();

    let r = Node::new(HashMap::new(), vec![], None, "h1".into());
    let c = Node::new(HashMap::new(), vec![], None, "p".into());
    Node::push(r.clone(), c.clone());
    Node::drain(c);

    assert_eq!(t, r.borrow().to_owned());
}

#[test]
fn test_replace() {
    let t = Node::de("<h1><n></n></h1>".into()).unwrap();

    let r = Node::new(HashMap::new(), vec![], None, "h1".into());
    let c = Node::new(HashMap::new(), vec![], None, "p".into());
    let n = Node::new(HashMap::new(), vec![], None, "n".into());
    Node::push(r.clone(), c.clone());
    c.replace(n.borrow().to_owned());

    assert_eq!(t, r.borrow().to_owned());
}
