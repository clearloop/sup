use crate::{
    result::{Error, Result},
    Config,
};
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
        substrate.push(format!(
            ".sup/{}/{}",
            config.node.name_space(),
            config.node.name()
        ));

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
        if String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.dir, "checkout", patt])
                .output()?
                .stderr,
        )
        .contains("error")
        {
            return Err(Error::Sup(format!("Checkout to tag {} failed", patt)));
        }

        Ok(())
    }

    /// Get current tag
    pub fn cur_tag(&self) -> Result<String> {
        let tag = String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.dir, "tag", "--points-at", "HEAD"])
                .output()?
                .stdout,
        )
        .trim()
        .to_string();

        Ok(if tag.contains('\n') {
            let tags = tag.split('\n').collect::<Vec<_>>();
            tags[tags.len() - 1].to_string()
        } else {
            tag
        })
    }

    /// Get the latest tag
    pub fn latest_tag(&self) -> Result<String> {
        // git describe --tags $(git rev-list --tags --max-count=1)
        let hashes = String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.dir, "rev-list", "--tags", "--max-count=1"])
                .output()?
                .stdout,
        )
        .trim()
        .to_string();

        Ok(String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.dir, "describe", "--tags", &hashes])
                .output()?
                .stdout,
        )
        .trim()
        .to_string())
    }

    /// List substrate tags
    pub fn tag(&self) -> Result<Vec<String>> {
        let mut tags = String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.dir, "tag", "--sort=-creatordate"])
                .output()?
                .stdout,
        )
        .to_string()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|t| t.starts_with('v') || t.starts_with('p'))
        .map(|t| t.to_string())
        .collect::<Vec<_>>();
        tags.reverse();

        Ok(tags)
    }
}
