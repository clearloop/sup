//! Comamnd Upgrade

use crate::{
    registry::{redep, Registry},
    result::{Error, Result},
};
use std::path::PathBuf;

/// Exec command `switch`
pub fn exec(path: PathBuf, mut tag: String, update: bool) -> Result<()> {
    let registry = Registry::new()?;
    let mut tags = registry.tag()?;
    if update || tags.is_empty() {
        println!("Fetching registry...");
        registry.update()?;
        tags = registry.tag()?;
    }

    if tag.is_empty() {
        tag = registry.latest_tag()?;
    } else if !tags.contains(&tag) {
        return Err(Error::Sup(format!(
            "The registry at {} doesn't have tag {}",
            registry.dir, tag,
        )));
    }

    // Checkout to the target tag
    registry.checkout(&tag)?;
    println!(
        "Switching to tag {} for {}",
        &tag,
        &path.to_str().unwrap_or(".")
    );

    // Operate tags
    let crates = etc::find_all(path, "Cargo.toml")?;
    for ct in crates {
        redep(&ct, &registry)?;
    }

    // Checkout back to the latest commit
    registry.checkout("master")?;
    Ok(())
}
