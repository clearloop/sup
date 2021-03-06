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

    if !info.contains("wasm32-unknown-unknown") && !info.contains("nightly-2020-10-05") {
        Command::new("rustup")
            .args(vec!["install", "nightly"])
            .status()?;
        Command::new("rustup")
            .args(vec!["override", "set", "nightly-2020-10-05"])
            .status()?;
        Command::new("rustup")
            .args(vec![
                "target",
                "add",
                "wasm32-unknown-unknown",
                "--toolchain",
                "nightly-2020-10-05",
            ])
            .status()?;
    }

    Ok(())
}

/// Exec command `new`
pub fn exec(
    mut registry: Registry,
    target: PathBuf,
    skip: bool,
    tag: Option<String>,
) -> Result<()> {
    if !skip {
        rustup()?;
    }

    // Checkout tag
    let tags = registry.tag()?;
    if tag.is_some() && tags.is_empty() {
        registry.update()?;
    }

    if let Some(tag) = &tag {
        registry.checkout(tag)?;
    }

    let substrate = Etc::from(&registry.dir);
    if let Ok(template) = substrate.find("node-template") {
        etc::cp_r(template, PathBuf::from(&target))?;

        // Create manifest
        let mani = workspace(&target, &registry)?;
        let mut dst = String::with_capacity(128);
        mani.serialize(Serializer::pretty(&mut dst).pretty_array(true))?;
        Etc::from(&target).open("Cargo.toml")?.write(dst)?;

        if let Some(tag) = &tag {
            println!(
                "Created node-template {:?} with tag {} succeed!",
                &target, &tag
            );
        } else {
            println!("Created node-template {:?} without tag succeed!", &target,);
        }
    } else {
        if let Some(tag) = &tag {
            println!(
                "The registry {} at tag {} doesn't have node-template",
                &registry.config.node.registry, &tag
            );
        } else {
            println!(
                "The registry {} without tag doesn't have node-template",
                &registry.config.node.registry
            )
        }
        println!("failed.");
        return Ok(());
    }

    // Checkout back to the latest commit
    registry.checkout("master")?;
    if let Some(tag) = tag {
        registry.config.node.tag(&tag);
    }

    registry.config.gen(target)?;
    println!("ok!");
    Ok(())
}
