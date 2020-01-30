use elvis::{Colors, Serde, Unit};

#[test]
fn test_colors() {
    let red = Colors::Red.to_hex();
    assert_eq!(red, "0xFFF44336".to_string());

    let wred = Colors::Red.ser();
    assert_eq!(wred, "rgba(244, 67, 54, 1.0)".to_string());

    let tred = Colors::de(wred).unwrap();
    assert_eq!(tred, Colors::ORGB(1.0, 244, 67, 54));
    assert_eq!(tred, Colors::Red);

    let dred = Colors::from_hex(red);
    assert_eq!(tred, dred);
    assert_ne!(Colors::Red, Colors::Blue);
}
