//! EPM Commands
use crate::template::APP_TEMPLATE;
use elvis_backend::Crate;
use std::{env, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

mod new;

#[derive(StructOpt)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Start develop server
    #[structopt(alias = "dev")]
    Dev {
        /// Use verbose output
        #[structopt(short, long)]
        verbose: bool,
    },
    /// Create a new elvis package in an existing directory
    #[structopt(alias = "init")]
    Init,
    /// Create a new elvis package
    #[structopt(alias = "new")]
    New {
        /// Package path
        #[structopt(name = "PATH")]
        path: PathBuf,
    },
}

/// Exec commands
pub fn exec() {
    let opt = Opt::from_args();
    match opt {
        Opt::Init => match env::current_dir() {
            Ok(p) => new::run(p, APP_TEMPLATE),
            Err(e) => error!("Exec epm init failed: {:?}", e),
        },
        Opt::New { path } => new::run(path, APP_TEMPLATE),
        Opt::Dev { verbose } => {
            if verbose {
                env_logger::from_env(env_logger::Env::default().default_filter_or("elvis")).init();
            } else {
                env_logger::from_env(env_logger::Env::new().default_filter_or("info"))
                    .format_timestamp(None)
                    .init();
            }
            match Crate::new() {
                Ok(c) => {
                    if let Err(e) = c.serve(3000) {
                        error!("Exec epm dev failed: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("Could not find elvis crate in current dir: {:?}", e);
                }
            }
        }
    }
}
