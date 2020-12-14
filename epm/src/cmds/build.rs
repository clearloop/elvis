//! Build crate
use elvis_backend::Crate;
use std::path::PathBuf;

/// Build crate to output dir
pub fn run(path: PathBuf) {
    match Crate::new() {
        Ok(mut c) => {
            if let Err(e) = c.out_dir(path).build() {
                error!("Exec epm build failed: {:?}", e);
            }
        }
        Err(e) => {
            error!("Could not find elvis crate in current dir: {:?}", e);
        }
    }
}
