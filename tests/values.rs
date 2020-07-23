use elvis::widgets::values::{Colors, Unit};
use elvis_shared::Serde;

#[test]
fn test_colors() {
    let red = Colors::Red.to_hex();
    assert_eq!(red, "0xFFF44336".to_string());

    let wred = Colors::Red.ser();
    assert_eq!(wred, "rgba(244, 67, 54, 1.0)".to_string());
}

#[test]
fn test_units() {
    let em = Unit::Em(1.0);
    assert_eq!(em.ser(), "1.0em");

    let per = Unit::Percent(100.0);
    assert_eq!(per.ser(), "100.0%");

    let nu = Unit::None(42.0);
    assert_eq!(nu.ser(), "42");
}
