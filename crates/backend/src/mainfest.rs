//! Crate mainfesst
use crate::err::Error;
use cargo_metadata::{Metadata, MetadataCommand};
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
    process::{Command, ExitStatus},
};
use strsim::levenshtein;
use wasm_bindgen_cli_support::Bindgen;

const WASM_PACK_METADATA_KEY: &str = "package.metadata.wasm-pack";

#[doc(hidden)]
#[derive(Deserialize)]
pub struct CargoManifest {
    package: CargoPackage,
}

/// Package
#[derive(Deserialize)]
struct CargoPackage {
    name: String,
}
/// Elvis crate
pub struct Crate {
    idx: usize,
    /// Build mode
    debug: bool,
    /// Crate Data
    data: Metadata,
    /// Crate root
    root: PathBuf,
    /// The out wasm path
    wasm: PathBuf,
}

pub struct ManifestAndUnsedKeys {
    pub manifest: CargoManifest,
    pub unused_keys: BTreeSet<String>,
}

impl Crate {
    /// New crate data
    pub fn new(root: PathBuf) -> Result<Crate, Error> {
        let manifest = root.join("Cargo.toml");
        let data = MetadataCommand::new()
            .manifest_path(&manifest)
            .exec()
            .unwrap();
        let mnk = Crate::parse_crate_data(&manifest)?;
        let idx = data
            .packages
            .iter()
            .position(|pkg| {
                pkg.name == mnk.manifest.package.name
                    && Crate::is_same_path(&pkg.manifest_path, &manifest)
            })
            .ok_or_else(|| Error::Custom("failed to find package in metadata".to_string()))?;

        Ok(Crate {
            idx,
            data,
            debug: true,
            root: root.to_path_buf(),
            wasm: root.join("pkg"),
        })
    }

    /// Reset debug mode
    pub fn debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

    /// Reset out dir
    pub fn out_dir(&mut self, dir: PathBuf) -> &mut Self {
        self.wasm = dir;
        self
    }

    /// Build the crate
    pub fn build(&self) -> Result<ExitStatus, Error> {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(&self.root);
        cmd.arg("build");

        if !self.debug {
            cmd.arg("--release");
        }

        Ok(cmd.status()?)
    }

    /// Compile wasm files
    pub fn bindgen(&self) -> Result<(), Error> {
        let mut b = Bindgen::new();
        let wasm = self
            .data
            .target_directory
            .join("wasm32-unknown-unknown")
            .join(match self.debug {
                true => "debug",
                false => "release",
            })
            .join(&self.name())
            .with_extension("wasm");

        b.input_path(wasm);
        if let Err(err) = b.web(true) {
            return Err(Error::Custom(err.to_string()));
        }

        if !self.debug {
            b.debug(false);
        }

        if let Err(err) = b.generate(&self.wasm) {
            return Err(Error::Custom(err.to_string()));
        }

        Ok(())
    }

    /// Watch the file system
    pub fn watch(&self) {
        println!("The build dir is {:?}", &self.wasm);
    }

    fn name(&self) -> String {
        let pkg = &self.data.packages[self.idx];
        match pkg
            .targets
            .iter()
            .find(|t| t.kind.iter().any(|k| k == "cdylib"))
        {
            Some(lib) => lib.name.replace("-", "_"),
            None => pkg.name.replace("-", "_"),
        }
    }

    fn is_same_path(lp: &PathBuf, rp: &PathBuf) -> bool {
        if let Ok(lp) = fs::canonicalize(&lp) {
            if let Ok(rp) = fs::canonicalize(&rp) {
                return lp == rp;
            }
        }
        lp == rp
    }

    /// Read the `manifest_path` file and deserializes it using the toml Deserializer.
    /// Returns a Result containing `ManifestAndUnsedKeys` which contains `CargoManifest`
    /// and a `BTreeSet<String>` containing the unused keys from the parsed file.
    ///
    /// # Errors
    /// Will return Err if the file (manifest_path) couldn't be read or
    /// if deserialize to `CargoManifest` fails.
    fn parse_crate_data(manifest_path: &PathBuf) -> Result<ManifestAndUnsedKeys, Error> {
        let manifest = fs::read_to_string(&manifest_path)?;
        let manifest = &mut toml::Deserializer::new(&manifest);

        let mut unused_keys = BTreeSet::new();
        let levenshtein_threshold = 1;

        let manifest: CargoManifest = serde_ignored::deserialize(manifest, |path| {
            let path_string = path.to_string();

            if path_string.starts_with("package.metadata")
                && (path_string.contains("wasm-pack")
                    || levenshtein(WASM_PACK_METADATA_KEY, &path_string) <= levenshtein_threshold)
            {
                unused_keys.insert(path_string);
            }
        })?;

        Ok(ManifestAndUnsedKeys {
            manifest,
            unused_keys,
        })
    }
}
