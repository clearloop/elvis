use elvis_backend::Crate;

fn main() {
    let pkg = Crate::new().unwrap();

    pkg.build().unwrap();
    pkg.bindgen().unwrap();
}
