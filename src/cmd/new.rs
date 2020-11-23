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

/// Check if need update rustup
pub fn rustup() -> Result<()> {
    let info = String::from_utf8_lossy(&Command::new("rustup").args(vec!["show"]).output()?.stdout)
        .to_string();

    if !info.contains("wasm32-unknown-unknown") && !info.contains("nightly-2020-11-20") {
        Command::new("rustup")
            .args(vec!["default", "nightly-11-20"])
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

    Ok(())
}

/// Exec command `new`
pub fn exec(target: PathBuf, skip: bool, mut tag: String) -> Result<()> {
    let has_tag = !tag.is_empty();

    // Fetch registry
    let registry = Registry::new()?;
    if !skip {
        rustup()?;
    }

    // Checkout tag
    let tags = registry.tag()?;
    if tags.is_empty() {
        registry.update()?;
    }

    if !has_tag || !tags.contains(&tag) {
        tag = registry.latest_tag()?;
    }

    registry.checkout(&tag)?;
    let substrate = Etc::from(&registry.dir);
    let template = substrate.find("node-template")?;
    etc::cp_r(template, PathBuf::from(&target))?;

    // Create manifest
    let mani = workspace(&target, &registry)?;
    let mut dst = String::with_capacity(128);
    mani.serialize(Serializer::pretty(&mut dst).pretty_array(true))?;
    Etc::from(&target).open("Cargo.toml")?.write(dst)?;
    println!("Created node-template {:?} succeed!", &target);

    // Checkout back to the latest commit
    registry.checkout("master")?;
    Ok(())
}
