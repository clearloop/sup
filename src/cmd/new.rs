//! Command `new`
use crate::{
    registry::{Manifest, Registry},
    result::Result,
};
use etc::{Etc, FileSystem, Write};
use serde::Serialize;
use std::path::PathBuf;
use toml::Serializer;

/// Genrate workspace
pub fn workspace(target: &PathBuf) -> Result<Manifest> {
    let crates = etc::find_all(target, "Cargo.toml")?;
    let ts = target.to_str().unwrap_or_default();
    let mut mani = Manifest::default();
    mani.workspace.members = crates
        .iter()
        .map(|c| {
            let ps = c.to_string_lossy();
            let start = ps.find(ts).unwrap_or(0) + ts.len();
            if ps.len() > (start + 12) {
                ps[(start + 1)..ps.len() - 11].to_string()
            } else {
                "".to_string()
            }
        })
        .filter(|s| !s.is_empty())
        .collect();

    Ok(mani)
}

/// Exec command `new`
pub fn exec(target: PathBuf) -> Result<()> {
    let registry = Registry::new()?;
    let substrate = Etc::from(&registry.0);
    let template = substrate.find("node-template")?;
    etc::cp_r(template, PathBuf::from(&target))?;

    let mani = workspace(&target)?;
    let mut dst = String::with_capacity(128);
    mani.serialize(Serializer::pretty(&mut dst).pretty_array(true))?;
    Etc::from(&target).open("Cargo.toml")?.write(dst)?;
    println!("Created node-template {:?} succeed!", &target);

    Ok(())
}
