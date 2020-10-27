//! Command Config
use crate::{registry::Registry, result::Result};
use std::process::Command;

/// Exec `config` command
pub fn exec(edit: bool) -> Result<()> {
    let registry = Registry::new()?;
    if edit {
        let mut home = dirs::home_dir().expect("Could not find home dir");
        home.push(".sup/config.toml");
        Command::new("vi")
            .arg(home.to_string_lossy().to_string())
            .status()?;
    } else {
        println!("{:#?}", registry.config);
    }
    Ok(())
}
