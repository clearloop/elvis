use elvis::{
    widgets::values::{Colors, Unit},
    Serde,
};

#[test]
fn test_colors() {
    let red = Colors::Red.to_hex();
    assert_eq!(red, "0xFFF44336".to_string());

    let wred = Colors::Red.ser();
    assert_eq!(wred, "rgba(244, 67, 54, 1.0)".to_string());

    let tred = Colors::de(wred).unwrap();
    assert_eq!(tred, Colors::ORGB(1.0, 244, 67, 54));
    assert_eq!(tred, Colors::Red);
    assert_eq!(Colors::Red, Colors::ORGB(1.0, 244, 67, 54));

    let dred = Colors::from_hex(red);
    assert_eq!(tred, dred);
    assert_ne!(Colors::Red, Colors::Blue);
}

#[test]
fn test_units() {
    let em = Unit::Em(1.0);
    assert_eq!(em.ser(), "1.0em");
    assert_eq!(em, Unit::de("1.0em".into()).unwrap());

    let per = Unit::Percent(100.0);
    assert_eq!(per.ser(), "100.0%");

    let nu = Unit::None(42.0);
    assert_eq!(nu.ser(), "42");
}
