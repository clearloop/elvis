use elvis_backend::Crate;
use std::path::PathBuf;

#[test]
fn test_build() {
    let root = PathBuf::from(".");
    let pkg = Crate::new(root).unwrap();

    // println!("{:?}", env::var("OUT_DIR").unwrap());
    pkg.build().unwrap();
    pkg.bindgen().unwrap();
}
