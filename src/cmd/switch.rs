//! Comamnd Upgrade

use crate::{
    registry::{redep, Registry},
    result::{Error, Result},
};
use std::path::PathBuf;

/// Exec command `upgrade`
pub fn exec(path: PathBuf, mut tag: String, update: bool) -> Result<()> {
    let registry = Registry::new()?;
    if update {
        println!("Fetching registry...");
        registry.update()?;
    }

    // tags
    let tags = registry.tag()?;
    if tag.is_empty() && !tags.is_empty() {
        tag = tags[tags.len() - 1].clone();
    } else if !tags.contains(&tag) {
        return Err(Error::Sup(format!(
            "The registry at {} doesn't have tag {}",
            registry.dir, tag,
        )));
    }

    // Checkout to the target tag
    registry.checkout(&tag)?;
    let crates = etc::find_all(path, "Cargo.toml")?;
    for ct in crates {
        redep(&ct, &registry)?;
    }

    // Checkout back to the latest commit
    registry.checkout("master")?;
    Ok(())
}
