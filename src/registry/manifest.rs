use serde::Deserialize;

/// Cargo Package
#[derive(Deserialize, Default)]
pub struct Package {
    /// Package name
    pub name: String,
}

/// Cargo manifest
#[derive(Deserialize, Default)]
pub struct Manifest {
    /// Cargo Package
    pub package: Package,
}
