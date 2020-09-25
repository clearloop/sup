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
        #[structopt(short, default_value = "10")]
        limit: usize,
    },
    /// Update registry
    Update,
}

/// Exec commands
pub fn exec() -> Result<()> {
    let opt = Opt::from_args();
    let registry = Registry::new().expect("woolala");
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
    }

    Ok(())
}
