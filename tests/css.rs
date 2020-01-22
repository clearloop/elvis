use elvis::CSS;

#[test]
fn test_as_ref() {
    let c = CSS("css".to_string());
    assert_eq!(c.as_ref(), "css");
}

#[test]
fn test_attr() {
    let mut c = CSS("".to_string());
    c.height("30px");
    assert_eq!(c.as_ref(), "height: 30px;");
}
