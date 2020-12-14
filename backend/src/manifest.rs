//! Cargo Manifest
use crate::{
    cargo::{CargoManifest, ManifestAndUnsedKeys},
    err::Error,
    html::{DEV_HTML_TEMPLATE, HTML_TEMPLATE},
    logger::Logger,
    server,
};
use cargo_metadata::{Metadata, MetadataCommand};
use etc::{Etc, FileSystem};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::prelude::*;
use std::{
    collections::BTreeSet,
    env, fs,
    path::PathBuf,
    process::{Command, ExitStatus},
    sync::mpsc::{channel, Sender},
    time::Duration,
};
use strsim::levenshtein;
use wasm_bindgen_cli_support::Bindgen;

const ELVIS_METADATA_KEY: &str = "package.metadata.elvis";

/// Elvis crate
#[derive(Clone)]
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

impl Crate {
    /// New crate data
    pub fn new() -> Result<Crate, Error> {
        let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let manifest = root.join("Cargo.toml");
        let data = MetadataCommand::new()
            .manifest_path(&manifest)
            .exec()
            .unwrap();

        // Get manifest data
        let mnk = Crate::parse_crate_data(&manifest)?;
        let idx = data
            .packages
            .iter()
            .position(|pkg| {
                pkg.name == mnk.manifest.package.name
                    && Crate::is_same_path(&pkg.manifest_path, &manifest)
            })
            .ok_or_else(|| Error::Custom("Failed to find package in metadata".to_string()))?;

        // create dirs
        let pkg = root.join("pkg");
        if !pkg.exists() {
            fs::create_dir_all(pkg)?;
        }

        Ok(Crate {
            idx,
            data,
            debug: true,
            root: root.clone(),
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

    /// Port the wasm path
    pub fn wasm(&self) -> &PathBuf {
        &self.wasm
    }

    /// Reset root dir
    pub fn root(&mut self, dir: PathBuf) -> &mut Self {
        self.root = dir;
        self
    }

    /// Compile the crate
    pub fn compile(&self) -> Result<ExitStatus, Error> {
        let mut cmd = Command::new("cargo");
        cmd.current_dir(&self.root);
        cmd.arg("build")
            .arg("--lib")
            .arg("--target")
            .arg("wasm32-unknown-unknown");

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

        // Generate .wasm file
        if let Err(err) = b.generate(&self.wasm) {
            return Err(Error::Custom(err.to_string()));
        }

        // Optimate wasm file size
        let mut wasm_path = self.wasm().to_path_buf();
        wasm_path.push(format!("{}{}", self.name().as_str(), "_bg.wasm"));
        if let Some(wp) = wasm_path.to_str() {
            match Crate::read_module(&wp) {
                Ok(mut module) => {
                    module.optimize(&binaryen::CodegenConfig {
                        optimization_level: 2,
                        shrink_level: 2,
                        debug_info: true,
                    });
                    let optimized_wasm = module.write();
                    Crate::write_module(&wp, &optimized_wasm)?
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    /// Compile crate and bindgen
    pub fn compile_and_bindgen(&self) -> Result<(), Error> {
        self.compile()?;
        self.bindgen()
    }

    /// Watch the file system
    pub fn watch(&self, wtx: Sender<bool>) -> Result<(), Error> {
        self.compile_and_bindgen()?;

        // channels
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
        watcher.watch(&self.root, RecursiveMode::Recursive)?;

        loop {
            match rx.recv() {
                Ok(event) => match event {
                    DebouncedEvent::Write(event) | DebouncedEvent::Remove(event) => {
                        if let Some(ext) = event.extension() {
                            if ext == "rs" {
                                if let Some(name) = event.file_name() {
                                    if event.exists() {
                                        trace!("write {:?}", name)
                                    } else {
                                        trace!("remove {:?}", name)
                                    }
                                }

                                self.compile_and_bindgen()?;
                                wtx.send(true).unwrap_or_default();
                            }
                        }
                    }
                    _ => {}
                },
                Err(e) => error!("{:?}", e),
            }
        }
    }

    /// Compile APP
    pub fn build(&self) -> Result<(), Error> {
        self.write_pages(HTML_TEMPLATE)?;
        self.compile()?;
        self.bindgen()
    }

    /// Serve APP
    pub fn serve(self, port: u16) -> Result<(), Error> {
        self.write_pages(DEV_HTML_TEMPLATE)?;
        if let Err(e) = server::run(self, port) {
            logger!(Logger::ServerStartFailed, e);
            return Err(Error::Custom(format!("{:?}", e)));
        }

        Ok(())
    }

    fn write_pages(&self, template: &str) -> Result<(), Error> {
        let codegen = |p: &str| {
            fs::write(
                &self.wasm.join(format!("{}.html", &p)),
                template
                    .replace("${entry}", &["/", &self.name(), ".js"].join(""))
                    .replace("${run}", &p),
            )
        };

        let pages = self.root.join("pages");
        if pages.exists() {
            for f in Etc::new(&self.root.join("pages"))?.ls()? {
                if f.ends_with(".rs") && f != "mod.rs" {
                    codegen(&f[..f.len() - 3])?;
                }
            }
        } else {
            codegen("index")?;
        }

        Ok(())
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
                && (path_string.contains("elvis")
                    || levenshtein(ELVIS_METADATA_KEY, &path_string) <= levenshtein_threshold)
            {
                unused_keys.insert(path_string);
            }
        })?;

        Ok(ManifestAndUnsedKeys { manifest })
    }

    fn read_module(filename: &str) -> Result<binaryen::Module, Error> {
        let mut f = File::open(filename)?;
        let mut contents = Vec::new();
        f.read_to_end(&mut contents)?;

        binaryen::Module::read(&contents).map_err(|_| Error::Custom("Empty result".to_string()))
    }

    fn write_module(filename: &str, wasm: &[u8]) -> Result<(), Error> {
        let mut f = File::create(filename)?;
        f.write_all(wasm).expect("failed to write file");
        Ok(())
    }
}
