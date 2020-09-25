//! Command `source`
use crate::{registry::Registry, result::Result};

/// Exec command `source`
pub fn exec(query: String) -> Result<()> {
    let registry = Registry::new()?;
    let source = registry.source()?;
    println!(
        "{}",
        if query.is_empty() {
            source
        } else {
            source
                .iter()
                .filter(|n| n.contains(&query))
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        }
        .join("\n")
    );
    Ok(())
}
