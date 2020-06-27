use serde::Deserialize;

#[derive(Deserialize)]
pub struct CargoManifest {
    pub package: CargoPackage,
}

/// Package
#[derive(Deserialize)]
pub struct CargoPackage {
    pub name: String,
}

pub struct ManifestAndUnsedKeys {
    pub manifest: CargoManifest,
    // unused_keys: BTreeSet<String>,
}
