use crate::result::Result;
use etc::{Etc, Read};
use std::process::Command;

mod manifest;

/// Substrate registry
pub struct Registry(
    /// Substrate git directory
    pub String,
);

impl Registry {
    /// New registry
    pub fn new() -> Result<Registry> {
        let mut substrate = dirs::home_dir().expect("Could not find home directory");
        substrate.push(".substrate");

        let registry = substrate.to_string_lossy().to_owned();
        if !substrate.exists() {
            Command::new("git")
                .args(vec![
                    "clone",
                    "https://github.com/paritytech/substrate.git",
                    &registry,
                    "--depth=1",
                ])
                .status()?;
        }

        Ok(Registry(registry.to_string()))
    }

    /// List crates
    pub fn source(&self) -> Result<Vec<String>> {
        Ok(etc::find_all(&self.0, "Cargo.toml")?
            .iter()
            .map(|mani| {
                toml::from_slice::<manifest::Manifest>(&Etc::from(mani).read().unwrap_or_default())
                    .unwrap_or_default()
                    .package
                    .name
            })
            .filter(|s| !s.is_empty())
            .collect())
    }

    /// Update registry
    pub fn update(&self) -> Result<()> {
        Command::new("git")
            .args(vec!["-C", &self.0, "pull", "origin", "master"])
            .status()?;

        Ok(())
    }

    /// List substrate tags
    pub fn tag(&self) -> Result<Vec<String>> {
        Ok(String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.0, "tag", "--list"])
                .output()?
                .stdout,
        )
        .to_string()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|t| t.starts_with('v'))
        .map(|t| t.to_string())
        .collect())
    }
}
