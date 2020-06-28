use elvis_backend::Crate;

#[test]
fn test_build() {
    let pkg = Crate::new().unwrap();

    pkg.build().unwrap();
    pkg.bindgen().unwrap();
}
