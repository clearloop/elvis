use elvis_core::Node;

#[test]
fn eq() {
    let a = Node::default();
    let b = Node::default();
    assert_eq!(a, b);
}
