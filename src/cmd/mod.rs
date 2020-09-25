// mod new;
use crate::Registry;
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
    /// Tag current substrate chain or list substrate available tags
    Tag {
        /// Enable while use this command to list substrate tags
        #[structopt(short, default_value = "10")]
        limit: usize,
    },
    /// Update registry
    Update,
}

/// Exec commands
pub fn exec() {
    let opt = Opt::from_args();
    let registry = Registry::new();
    match opt {
        Opt::New { path } => {
            let substrate = Etc::from(&registry.0);
            let template = substrate
                .find("node-template")
                .expect(&format!("Could not find node-template in {:?}", registry.0));
            etc::cp_r(template, PathBuf::from(&path)).expect("Generate node-template failed");
            println!("Created node-template {:?} succeed!", &path);
        }
        Opt::Tag { limit } => {
            let mut tags = registry.tag();
            let last = if limit < tags.len() || limit < 1 {
                limit
            } else {
                tags.len()
            };

            tags.reverse();
            println!("{:?}", &tags[..last]);
        }
        Opt::Update => {
            println!("Fetching registry...");
            registry.update()
        }
    }
}
