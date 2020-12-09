//! Comamnd Update
use crate::{
    registry::{redep, Registry},
    result::Result,
};
use etc::{Etc, FileSystem};
use std::path::PathBuf;

/// Exec command `switch`
pub fn exec(mut registry: Registry, path: PathBuf, tag: Option<String>) -> Result<()> {
    let mut tags = registry.tag()?;
    if tags.is_empty() {
        println!("Fetching registry...");
        registry.update()?;
        tags = registry.tag()?;
    }

    if let Some(ref tag) = tag {
        if !tags.contains(&tag) {
            println!(
                "Doesn't have tag {} in registry, please retry with `--update` flag",
                &tag
            );
            return Ok(());
        } else {
            // Checkout to the target tag
            registry.checkout(&tag)?;
            println!(
                "Switching to tag {} for {}",
                &tag,
                &path.to_str().unwrap_or(".")
            );
        }

        registry.config.node.tag(tag);
    } else {
        if Etc::from(&registry.dir).find("node-template").is_err() {
            println!(
                "Registry {} with tag {:?} doesn't have node-template, \
                 update {:?} failed, please retry with `--update` flag",
                &registry.config.node.registry, &tag, &path
            );
            println!("failed.");
            return Ok(());
        }

        // If has tag field in current `.sup.toml`
        if let Some(ref tag) = registry.config.node.tag {
            // Checkout to the target tag
            if registry.checkout(&tag).is_err() {
                println!(
                    "Doesn't have tag {} in registry, please retry with `--update` flag",
                    &tag
                );
                println!("failed.");
                return Ok(());
            }

            println!("Update {} with tag {}", path.to_str().unwrap_or("."), &tag);
        } else {
            println!(
                "Use the latest registry without tag for {}",
                path.to_str().unwrap_or(".")
            );
        }
    }

    // Operate tags
    etc::find_all(&path, "Cargo.toml")?
        .iter()
        .map(|ct| redep(&ct, &registry))
        .collect::<Result<Vec<_>>>()?;

    // Checkout back to the latest commit
    registry.checkout("master")?;
    registry.config.gen(path)?;
    println!("ok!");
    Ok(())
}
