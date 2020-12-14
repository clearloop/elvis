//! Dev command
use elvis_backend::Crate;

/// Start development server
pub fn run(port: u16, verbose: bool) {
    if verbose {
        env_logger::from_env(env_logger::Env::default().default_filter_or("elvis")).init();
    } else {
        env_logger::from_env(env_logger::Env::new().default_filter_or("info"))
            .format_timestamp(None)
            .init();
    }
    match Crate::new() {
        Ok(c) => {
            if let Err(e) = c.serve(port) {
                error!("Exec epm dev failed: {:?}", e);
            }
        }
        Err(e) => {
            error!("Could not find elvis crate in current dir: {:?}", e);
        }
    }
}
