use crate::template::APP_TEMPLATE;
use elvis_backend::Crate;
use std::{env, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

mod new;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Start develop server
    #[structopt(alias = "dev")]
    Dev,
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
        Opt::Init => {
            new::run(env::current_dir().unwrap(), APP_TEMPLATE);
        }
        Opt::New { path } => {
            new::run(path, APP_TEMPLATE);
        }
        Opt::Dev => Crate::new().unwrap().serve().unwrap(),
    }
}
