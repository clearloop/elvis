use elvis::{Colors, Serde, TextStyle, Unit};

#[test]
fn test_text_style() {
    let ts = TextStyle::new(
        true,
        Colors::Red,
        true,
        Unit::Rem(1.0),
        Unit::None(700.0),
        Unit::Rem(1.0),
        Unit::Percent(100.0),
    );

    let sr = "color: rgba(244, 67, 54, 1.0); font-weight: 700; font-style: italic; font-size: 1.0rem; height: 1.0rem; font-stretch: 100.0%".to_string();
    assert_eq!(ts, TextStyle::de(sr).unwrap());
}
