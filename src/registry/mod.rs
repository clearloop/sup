use crate::result::Result;
use std::process::Command;

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
        let mut git = Command::new("git");
        if !substrate.exists() {
            git.args(vec![
                "clone",
                "https://github.com/paritytech/substrate.git",
                &registry,
                "--depth=1",
            ])
            .status()?;
        }

        Ok(Registry(registry.to_string()))
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
