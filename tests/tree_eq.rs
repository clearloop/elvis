use elvis_shared::Node;

#[test]
fn eq() {
    let a = Node::default();
    let b = Node::default();
    assert_eq!(a, b);
}
