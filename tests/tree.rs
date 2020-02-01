use elvis::{Serde, Tree};
use std::collections::HashMap;

#[test]
fn test_push() {
    let t = Tree::de("<h1><p></p></h1>".into()).unwrap();

    let r = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "h1");
    let c = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "p");
    Tree::push(r.clone(), c);
    assert_eq!(t, r.borrow().to_owned());
}

#[test]
fn test_remove() {
    let t = Tree::de("<h1></h1>".into()).unwrap();

    let r = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "h1");
    let c = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "p");
    Tree::push(r.clone(), c.clone());

    r.borrow_mut().remove(c);
    assert_eq!(t, r.borrow().to_owned());
}

#[test]
fn test_drain() {
    let t = Tree::de("<h1></h1>".into()).unwrap();

    let r = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "h1");
    let c = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "p");
    Tree::push(r.clone(), c.clone());
    Tree::drain(c);

    assert_eq!(t, r.borrow().to_owned());
}

#[test]
fn test_replace() {
    let t = Tree::de("<h1><n></n></h1>".into()).unwrap();

    let r = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "h1");
    let c = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "p");
    let n = Tree::new(HashMap::new(), HashMap::new(), vec![], None, "n");
    Tree::push(r.clone(), c.clone());
    c.replace(n.borrow().to_owned());

    assert_eq!(t, r.borrow().to_owned());
}
