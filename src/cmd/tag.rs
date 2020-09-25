//! Command `tag`
use crate::{registry::Registry, result::Result};

/// Exec comamnd `tag`
pub fn exec(limit: usize) -> Result<()> {
    let registry = Registry::new()?;
    let mut tags = registry.tag()?;
    let last = if limit < tags.len() || limit < 1 {
        limit
    } else {
        tags.len()
    };

    tags.reverse();
    println!("{}", &tags[..last].join("\n"));
    Ok(())
}
