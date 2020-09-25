//! Command `update`
use crate::{registry::Registry, result::Result};

/// Exec `update` command
pub fn exec() -> Result<()> {
    let registry = Registry::new()?;
    println!("Fetching registry...");
    registry.update()?;
    Ok(())
}
