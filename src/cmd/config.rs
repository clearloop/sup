//! Command Config
use crate::{
    registry::Registry,
    result::{Error, Result},
};
use etc::{Etc, FileSystem, Write};
use std::{path::PathBuf, process::Command};

/// Exec `config` command
pub fn exec(edit: bool, git: String) -> Result<()> {
    let mut registry = Registry::new()?;
    let mut should_rewrite = false;
    let cur_registry = PathBuf::from(&registry.dir);
    let home = cur_registry
        .parent()
        .expect("Could not find the home of sup");

    // If with `-e` flag, just edit the config file.
    if edit {
        Command::new("vi")
            .arg(home.to_string_lossy().to_string())
            .status()?;
        return Ok(());
    }

    // If with registry flag, reset the registry
    if !git.is_empty() && git.ne(&registry.config.node.registry) {
        if !git.ends_with(".git") {
            return Err(Error::Sup(format!("Wrong git url: {}", git)));
        }
        should_rewrite = true;
        registry.config.node.registry = git;
    }

    // Re-write the config file
    if should_rewrite {
        Etc::from(&home)
            .open("config.toml")?
            .write(toml::to_string(&registry.config)?)?;

        return Ok(());
    }

    println!("{:#?}", registry.config);
    Ok(())
}
