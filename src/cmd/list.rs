//! Command List
use crate::{registry::Registry, result::Result};
use std::cmp::Ordering;
use structopt::StructOpt;

const MAX_PACKAGE_NAME_LEN: usize = 50;

/// List registry source or tags
#[derive(Debug, StructOpt)]
pub enum List {
    /// List source crates of registry
    Source {
        /// Filter by tag
        #[structopt(short, long)]
        tag: Option<String>,
    },
    /// List tags of registry
    Tag {
        /// Limit the number of tags
        #[structopt(short, long, default_value = "10")]
        limit: usize,
    },
}

fn cap(mut name: String) -> String {
    name.push_str(&" ".repeat(MAX_PACKAGE_NAME_LEN - name.len()));
    name
}

/// Exec command `list`
pub fn exec(registry: Registry, list: List) -> Result<()> {
    match list {
        List::Source { tag } => {
            let mut should_checkout = false;
            let mut source = registry.source()?;
            source.sort_by(|(np, _), (nq, _)| np.partial_cmp(nq).unwrap_or(Ordering::Greater));

            if let Some(tag) = tag {
                if !tag.is_empty() {
                    should_checkout = true;
                    registry.checkout(&tag)?;
                }
            }

            // Prints source
            println!(
                "{}",
                source
                    .iter()
                    .map(|(name, ver)| { format!("{} - {}", cap(name.to_string()), ver) })
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            // Checkout back to the latest commit
            if should_checkout {
                registry.checkout("master")?;
            }
        }
        List::Tag { limit } => {
            // Get tags
            let mut tags = registry.tag()?;
            if tags.is_empty() {
                registry.update()?;
            }

            let last = if limit < tags.len() || limit < 1 {
                limit
            } else {
                tags.len()
            };

            tags.reverse();
            println!("{}", &tags[..last].join("\n"));
        }
    }

    Ok(())
}
