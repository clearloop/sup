//! Sup Commands
use crate::{registry::Registry, result::Result};
use std::path::PathBuf;
use structopt::StructOpt;

pub mod config;
pub mod list;
pub mod new;
pub mod update;

#[derive(StructOpt, Debug)]
enum Command {
    /// Shows or edits the current config
    Config {
        #[structopt(subcommand)]
        config: config::Config,
    },
    /// List registry source or tags
    List {
        #[structopt(subcommand)]
        list: list::List,
    },
    /// Create a new substrate node template
    New {
        /// Project path
        #[structopt(name = "PATH")]
        path: PathBuf,
        /// Skip rust toolchain check
        #[structopt(short, long)]
        skip: bool,
        /// Specify a tag to generate
        #[structopt(short, long, default_value = "v2.0.0")]
        tag: String,
    },
    /// Update the target substrate project
    Update {
        /// Project path
        #[structopt(short, long, default_value = ".")]
        project: PathBuf,
        /// Registry tag
        #[structopt(short, long, default_value = "")]
        tag: String,
    },
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Pulls and updates the global registry
    #[structopt(short, long)]
    pull: bool,
    #[structopt(subcommand)]
    command: Command,
}

/// Exec commands
pub fn exec() -> Result<()> {
    let opt = Opt::from_args();

    // Check if update
    let registry = Registry::new()?;
    if opt.pull {
        registry.update()?;
    }

    // Match subcommands
    match opt.command {
        Command::Config { config } => config::exec(registry, config)?,
        Command::List { list } => list::exec(registry, list)?,
        Command::New { path, skip, tag } => new::exec(registry, path, skip, tag)?,
        Command::Update { project, tag } => update::exec(registry, project, tag)?,
    }

    Ok(())
}
