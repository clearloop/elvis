//! Cargo Manifest
use crate::{
    cargo::{CargoManifest, ManifestAndUnsedKeys},
    err::Error,
    html::DEV_HTML_TEMPLATE,
};
use cargo_metadata::{Metadata, MetadataCommand};
use futures::{join, sink::SinkExt, StreamExt};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::BTreeSet,
    env, fs,
    path::PathBuf,
    process::{Command, ExitStatus},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    time::Duration,
};
use strsim::levenshtein;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter,
};
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
        let root = env::current_dir().unwrap_or(PathBuf::from("."));
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
            .ok_or_else(|| Error::Custom("failed to find package in metadata".to_string()))?;

        // create dirs
        let pkg = root.join("pkg");
        if !pkg.exists() {
            fs::create_dir_all(pkg)?;
        }

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

    /// Reset root dir
    pub fn root(&mut self, dir: PathBuf) -> &mut Self {
        self.root = dir;
        self
    }

    /// Build the crate
    pub fn build(&self) -> Result<ExitStatus, Error> {
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

        if let Err(err) = b.generate(&self.wasm) {
            return Err(Error::Custom(err.to_string()));
        }

        Ok(())
    }

    /// Build crate and bindgen
    pub fn build_and_bindgen(&self) -> Result<(), Error> {
        self.build()?;
        self.bindgen()
    }

    /// Watch the file system
    pub fn watch(&self, wtx: Sender<bool>) -> Result<(), Error> {
        self.build_and_bindgen()?;

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
                                self.build_and_bindgen()?;
                                wtx.send(true).unwrap_or_default();
                            }
                        }
                    }
                    _ => {}
                },
                Err(e) => println!("watcher error: {:?}", e),
            }
        }
    }

    /// Pre-serve app
    fn pre_serve(&self) -> Result<(), Error> {
        self.build_and_bindgen()?;
        fs::write(
            &self.wasm.join("index.html"),
            DEV_HTML_TEMPLATE.replace("${entry}", &["/", &self.name(), ".js"].join("")),
        )?;
        Ok(())
    }

    /// Handle the updater
    async fn client_connect(ws: WebSocket, rx: Arc<Mutex<Receiver<bool>>>) {
        let (mut tx, mut crx) = ws.split();

        while let Some(_) = crx.next().await {
            if rx.lock().unwrap().recv().is_ok() {
                println!("hello, world");
                if let Err(e) = tx.send(Message::text("hello")).await {
                    eprintln!("websocket err :{:?}", e);
                }
            }
        }
        // Need to check this `unwrap`
        tokio::task::spawn_blocking(move || loop {
            if rx.lock().unwrap().recv().is_ok() {
                println!("hello, world");
                futures::poll(tx.send(Message::text("hello")));
            }
        });
    }

    /// Serve the backend
    #[tokio::main]
    async fn tokio_serve(self) -> Result<(), Error> {
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        let rx = warp::any().map(move || rx.clone());

        let index = warp::filters::fs::dir(self.wasm.clone());
        let updater = warp::path("updater")
            .and(warp::ws())
            .and(rx)
            .map(|ws: Ws, rx| ws.on_upgrade(move |socket| Self::client_connect(socket, rx)));

        // dev http server
        let server = warp::serve(index.or(updater)).run(([0, 0, 0, 0], 3000));

        // file watcher
        let watcher = tokio::task::spawn_blocking(move || self.watch(tx));
        if let Err(e) = join!(watcher, server).0 {
            return Err(Error::Custom(e.to_string()));
        }

        Ok(())
    }

    /// Serve APP
    pub fn serve(self) -> Result<(), Error> {
        &self.pre_serve()?;
        if let Err(_) = self.tokio_serve() {
            return Err(Error::Custom("tokio error".into()));
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
}
