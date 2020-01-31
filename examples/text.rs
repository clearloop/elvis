use elvis::{Serde, TextStyle};

fn main() {
    let sr = "color: rgba(244, 67, 54, 1.0); font-weight: 700; font-style: italic; font-size: 1.0rem; height: 1.0rem; font-stretch: 100.0%".to_string();
    println!("{:?}", TextStyle::de(sr));
}
