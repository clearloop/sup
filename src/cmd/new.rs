//! Command `new`
use crate::{
    registry::{redep, Manifest, Registry},
    result::Result,
};
use etc::{Etc, FileSystem, Write};
use serde::Serialize;
use std::{path::PathBuf, process::Command};
use toml::Serializer;

/// Genrate workspace
pub fn workspace(target: &PathBuf, registry: &Registry) -> Result<Manifest> {
    let crates = etc::find_all(target, "Cargo.toml")?;
    let ts = target.to_str().unwrap_or_default();
    let mut mani = Manifest::default();
    let mut members = vec![];
    for ct in crates {
        let ps = ct.to_string_lossy();
        if ps.contains("/target/") {
            continue;
        }

        // Redirect deps
        redep(&ct, registry)?;

        // Register path
        let begin = ps.find(ts).unwrap_or(0) + ts.len();
        if ps.len() > (begin + 12) {
            members.push(ps[(begin + 1)..ps.len() - 11].to_string())
        }
    }

    mani.workspace.members = members;
    Ok(mani)
}

/// Exec command `new`
pub fn exec(target: PathBuf, skip: bool) -> Result<()> {
    // Check wasm
    if !skip {
        Command::new("rustup")
            .args(vec!["install", "nightly"])
            .status()?;
        Command::new("rustup")
            .args(vec![
                "target",
                "add",
                "wasm32-unknown-unknown",
                "--toolchain",
                "nightly",
            ])
            .status()?;
    }

    // Fetch registry
    let registry = Registry::new()?;
    let substrate = Etc::from(&registry.dir);
    let template = substrate.find("node-template")?;
    etc::cp_r(template, PathBuf::from(&target))?;

    // Create manifest
    let mani = workspace(&target, &registry)?;
    let mut dst = String::with_capacity(128);
    mani.serialize(Serializer::pretty(&mut dst).pretty_array(true))?;
    Etc::from(&target).open("Cargo.toml")?.write(dst)?;
    println!("Created node-template {:?} succeed!", &target);

    Ok(())
}
