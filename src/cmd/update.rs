//! Comamnd Update

use crate::{
    registry::{redep, Registry},
    result::Result,
};
use std::path::PathBuf;

/// Exec command `switch`
pub fn exec(registry: Registry, path: PathBuf, tag: Option<String>) -> Result<()> {
    let mut tags = registry.tag()?;
    if tags.is_empty() {
        println!("Fetching registry...");
        registry.update()?;
        tags = registry.tag()?;
    }

    if let Some(tag) = tag {
        if !tags.contains(&tag) {
            println!(
                "Doesn't have tag {} in registry, retry with `--update` flag",
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
    } else {
        println!(
            "Use the latest registry without tags for {}",
            path.to_str().unwrap_or(".")
        );
    }

    // Operate tags
    let crates = etc::find_all(path, "Cargo.toml")?;
    for ct in crates {
        redep(&ct, &registry)?;
    }

    // Checkout back to the latest commit
    registry.checkout("master")?;
    Ok(())
}
