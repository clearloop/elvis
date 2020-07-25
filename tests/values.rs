use elvis::value::{Colors, Unit};

#[test]
fn test_colors() {
    let red = Colors::Red.to_hex();
    assert_eq!(red, "0xFFF44336".to_string());

    let wred = Colors::Red.to_string();
    assert_eq!(wred, "rgba(244, 67, 54, 1.0)".to_string());
}

#[test]
fn test_units() {
    let em = Unit::Em(1.0);
    assert_eq!(em.to_string(), "1.0em");

    let per = Unit::Percent(100.0);
    assert_eq!(per.to_string(), "100.0%");

    let nu = Unit::None(42.0);
    assert_eq!(nu.to_string(), "42");
}
