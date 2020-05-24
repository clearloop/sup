mod new;
mod node_template;

use node_template::NODE_TEMPLATE;
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
    // /// Update substrate dependencies listed in Cargo.toml
    // #[structopt(alias = "update")]
    // Update,
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
    }
}
