//! Sup Commands
use crate::result::Result;
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};

pub mod new;
pub mod source;
pub mod tag;
pub mod update;

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::InferSubcommands)]
enum Opt {
    /// Create a new substrate template
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
    match opt {
        Opt::New { path } => new::exec(path)?,
        Opt::Tag { limit } => tag::exec(limit)?,
        Opt::Update => update::exec()?,
        Opt::Source { query } => source::exec(query)?,
    }

    Ok(())
}
