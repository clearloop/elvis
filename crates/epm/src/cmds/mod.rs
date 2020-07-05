//! EPM Commands
use crate::template::APP_TEMPLATE;
use std::{path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

mod build;
mod dev;
mod init;
mod new;

#[derive(StructOpt)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Start development server
    #[structopt(alias = "dev")]
    Dev {
        /// Http server port
        #[structopt(short, long, default_value = "3000")]
        port: u16,
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
    Build {
        /// Output path
        #[structopt(name = "PATH")]
        output: PathBuf,
    }
}

/// Exec commands
pub fn exec() {
    let opt = Opt::from_args();
    match opt {
        Opt::Init => init::run(),
        Opt::New { path } => new::run(path, APP_TEMPLATE),
        Opt::Dev { port, verbose } => dev::run(port, verbose),
        Opt::Build { output } => build::run(output),
    }
}
