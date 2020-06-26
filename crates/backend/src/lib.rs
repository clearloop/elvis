use std::{
    path::PathBuf,
    process::{Child, Command},
};

mod err;
use err::Error;

/// Debug or Release
#[derive(Debug, PartialEq)]
pub enum BuildMode {
    Debug,
    Release,
}

/// Elvis crate
pub struct Crate {
    pub dir: PathBuf,
    pub out: PathBuf,
    pub mode: BuildMode,
}

/// Build the crate
pub fn build(pkg: PathBuf, mode: BuildMode) -> Result<Child, Error> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(pkg);
    cmd.arg("build");

    if mode == BuildMode::Release {
        cmd.arg("--release");
    }

    Ok(cmd.spawn()?)
}

/// Compile wasm files
pub fn compile(dir: PathBuf, out: PathBuf) {
    println!("The path is {:?}, the mode is {:?}", dir, out);
}

/// Watch the file system
pub fn watch<F>(dir: PathBuf, f: F)
where
    F: FnOnce() -> (),
{
    f();
    println!("The build dir is {:?}", dir);
}
