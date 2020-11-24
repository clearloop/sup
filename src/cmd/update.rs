//! Comamnd Update

use crate::{
    registry::{redep, Registry},
    result::Result,
};
use std::path::PathBuf;

/// Exec command `switch`
pub fn exec(path: PathBuf, tag: String) -> Result<()> {
    let registry = Registry::new()?;
    let mut tags = registry.tag()?;
    if tags.is_empty() {
        println!("Fetching registry...");
        registry.update()?;
        tags = registry.tag()?;
    }

    if !tag.is_empty() && tags.contains(&tag) {
        // Checkout to the target tag
        registry.checkout(&tag)?;
        println!(
            "Switching to tag {} for {}",
            &tag,
            &path.to_str().unwrap_or(".")
        );
    } else if tag.is_empty() {
        println!(
            "Use the latest registry without tags for {}",
            path.to_str().unwrap_or(".")
        );
    } else {
        println!(
            "Doesn't have tag {} in registry, retry with `--update` flag",
            &tag
        );
        return Ok(());
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
