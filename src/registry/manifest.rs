use serde::{Deserialize, Serialize};

/// Cargo Package
#[derive(Serialize, Deserialize, Default)]
pub struct Package {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
}

/// Cargo manifest
#[derive(Serialize, Deserialize, Default)]
pub struct Manifest {
    /// Cargo Package
    #[serde(skip_serializing)]
    pub package: Package,
    /// Cargo workspace
    #[serde(default)]
    pub workspace: WorkSpace,
}

/// Cargo workspace
#[derive(Serialize, Deserialize, Default)]
pub struct WorkSpace {
    /// Members of workspace
    pub members: Vec<String>,
}
