use crate::{result::Result, Config};
use etc::{Etc, Read};
use std::process::Command;

mod manifest;
mod redep;

pub use self::{manifest::Manifest, redep::redirect as redep};

/// Substrate registry
pub struct Registry {
    /// Config
    pub config: Config,
    /// Substrate git directory
    pub dir: String,
}

impl Registry {
    /// New registry
    pub fn new() -> Result<Registry> {
        let config = Config::new()?;
        let mut substrate = dirs::home_dir().expect("Could not find home directory");
        substrate.push(format!(".sup/{}", config.node.name()));

        let registry = substrate.to_string_lossy().to_owned();
        if !substrate.exists() {
            Command::new("git")
                .args(vec!["clone", &config.node.registry, &registry, "--depth=1"])
                .status()?;
        }

        Ok(Registry {
            config,
            dir: registry.to_string(),
        })
    }

    /// List crates
    pub fn source(&self) -> Result<Vec<(String, String)>> {
        Ok(etc::find_all(&self.dir, "Cargo.toml")?
            .iter()
            .map(|mani| {
                let pkg = toml::from_slice::<manifest::Manifest>(
                    &Etc::from(mani).read().unwrap_or_default(),
                )
                .unwrap_or_default()
                .package;
                (pkg.name, pkg.version)
            })
            .filter(|(name, _)| !name.is_empty() && !name.contains("node-template"))
            .collect())
    }

    /// Update registry
    pub fn update(&self) -> Result<()> {
        Command::new("git")
            .args(vec!["-C", &self.dir, "pull", "origin", "master", "--tags"])
            .status()?;

        Ok(())
    }

    /// Checkout to target tag
    pub fn checkout(&self, patt: &str) -> Result<()> {
        Command::new("git")
            .args(vec!["-C", &self.dir, "checkout", patt])
            .status()?;

        Ok(())
    }

    /// List substrate tags
    pub fn tag(&self) -> Result<Vec<String>> {
        Ok(String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.dir, "tag", "--list"])
                .output()?
                .stdout,
        )
        .to_string()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|t| t.starts_with('v'))
        .map(|t| t.to_string())
        .collect())
    }
}
