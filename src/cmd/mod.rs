//! Sup Commands
use crate::{result::Result, Registry};
use etc::{Etc, FileSystem};
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Create a new substrate package
    New {
        /// Package path
        #[structopt(name = "PATH")]
        path: PathBuf,
    },
    /// List available tags or apply tag to the current project
    Tag {
        /// Avaiable while using this command to list tags
        #[structopt(short, long, default_value = "10")]
        limit: usize,
    },
    /// Update registry
    Update,
    /// List Source
    Source {
        #[structopt(short, long, default_value = "")]
        query: String,
    },
}

/// Exec commands
pub fn exec() -> Result<()> {
    let opt = Opt::from_args();
    let registry = Registry::new().expect("Create registry failed");
    match opt {
        Opt::New { path } => {
            let substrate = Etc::from(&registry.0);
            let template = substrate.find("node-template")?;
            etc::cp_r(template, PathBuf::from(&path))?;
            println!("Created node-template {:?} succeed!", &path);
        }
        Opt::Tag { limit } => {
            let mut tags = registry.tag()?;
            let last = if limit < tags.len() || limit < 1 {
                limit
            } else {
                tags.len()
            };

            tags.reverse();
            println!("{}", &tags[..last].join("\n"));
        }
        Opt::Update => {
            println!("Fetching registry...");
            registry.update()?;
        }
        Opt::Source { query } => {
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
        }
    }

    Ok(())
}
