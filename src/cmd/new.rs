//! Command `new`
use crate::{registry::Registry, result::Result};
use etc::{Etc, FileSystem};
use std::path::PathBuf;

/// Exec command `new`
pub fn exec(path: PathBuf) -> Result<()> {
    let registry = Registry::new()?;
    let substrate = Etc::from(&registry.0);
    let template = substrate.find("node-template")?;
    etc::cp_r(template, PathBuf::from(&path))?;
    println!("Created node-template {:?} succeed!", &path);
    Ok(())
}
