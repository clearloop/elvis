// use elvis::{
//     widgets::values::{Colors, Unit},
//     widgets::TextStyle,
// };
// use elvis_shared::Serde;
//
// #[test]
// fn test_text_style() {
//     let ts = TextStyle {
//         bold: true,
//         color: Colors::Red,
//         italic: true,
//         height: Unit::Rem(1.0),
//         weight: Unit::None(700.0),
//         size: Unit::Rem(1.0),
//         stretch: Unit::Percent(100.0),
//     };
//
//     let sr = "color: rgba(244, 67, 54, 1.0); font-weight: 700; font-style: italic; font-size: 1.0rem; height: 1.0rem; font-stretch: 100.0%".to_string();
//     assert_eq!(ts, TextStyle::de(sr).unwrap());
// }
