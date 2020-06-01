mod new;

use crate::registry::{NODE_TEMPLATE, SUBSTRATE_TAGS};
use std::{env, path::PathBuf};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Create a new substrate package in an existing directory
    #[structopt(alias = "init")]
    Init,
    /// Create a new substrate package
    #[structopt(alias = "new")]
    New {
        /// Package path
        #[structopt(name = "PATH")]
        path: PathBuf,
    },
    /// List substrate available tags
    #[structopt(alias = "tags")]
    Tags,
}

/// Exec commands
pub fn exec() {
    let opt = Opt::from_args();
    match opt {
        Opt::Init => {
            new::run(env::current_dir().unwrap(), NODE_TEMPLATE);
        }
        Opt::New { path } => {
            new::run(path, NODE_TEMPLATE);
        }
        Opt::Tags => SUBSTRATE_TAGS[1..SUBSTRATE_TAGS.len() - 1]
            .replace("\"", "")
            .split(", ")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|t| println!("{}", t)),
    }
}
