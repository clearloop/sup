// mod new;
use crate::Registry;
use etc::Etc;
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
}

/// Exec commands
pub fn exec() {
    let opt = Opt::from_args();
    let registry = Registry::new();
    match opt {
        Opt::New { path } => {
            let mut substrate = registry.0;
            substrate.push_str("/bin/node-template");

            // copy substrate to target path
            let mut tree = Etc::from(PathBuf::from(substrate)).tree().unwrap();
            tree.load().unwrap();
            if let Some(mut children) = tree.children {
                children
                    .iter_mut()
                    .for_each(|t| t.redir(path.clone()).unwrap());
            }

            println!("Created node-template {:?} succeed!", &path);
        }
        Opt::Tag { limit } => {
            let mut tags = registry.list_tags();
            let last = if limit < tags.len() || limit < 1 {
                limit
            } else {
                tags.len()
            };

            tags.reverse();
            println!("{:?}", &tags[..last]);
        }
    }
}
