use elvis_backend::{build, BuildMode};
use std::path::PathBuf;

#[test]
fn test_build() {
    build(PathBuf::from("."), BuildMode::Debug).unwrap();
}
