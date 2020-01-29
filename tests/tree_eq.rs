use elvis::Tree;

#[test]
fn eq() {
    let a = Tree::default();
    let b = Tree::default();
    assert_eq!(a, b);
}
