//! Sup Commands
use crate::{registry::Registry, result::Result};
use std::path::PathBuf;
use structopt::StructOpt;

pub mod config;
pub mod list;
pub mod new;
pub mod source;
pub mod tag;
pub mod update;

#[derive(StructOpt, Debug)]
enum Command {
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
    /// List available tags
    Tag {
        /// The limit count of tags
        #[structopt(short, long, default_value = "10")]
        limit: usize,
        /// If update registry
        #[structopt(short, long)]
        update: bool,
    },

    /// List source crates
    Source {
        /// Show dependencies contain <query>
        #[structopt(short, long, default_value = "")]
        query: String,
        /// Registry tag
        #[structopt(short, long, default_value = "")]
        tag: String,
        /// If show versions
        #[structopt(short, long)]
        version: bool,
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
    /// Shows or edits the current config
    Config {
        #[structopt(subcommand)]
        config: config::Config,
    },
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Updates the global registry
    #[structopt(short, long)]
    update: bool,
    #[structopt(subcommand)]
    command: Command,
}

/// Exec commands
pub fn exec() -> Result<()> {
    let opt = Opt::from_args();

    // Check if update
    let registry = Registry::new()?;
    if opt.update {
        registry.update()?;
    }

    // Match subcommands
    match opt.command {
        Command::New { path, skip, tag } => new::exec(registry, path, skip, tag)?,
        Command::Config { config } => config::exec(registry, config)?,
        Command::Tag { limit, update } => tag::exec(registry, limit, update)?,
        Command::Update { project, tag } => update::exec(registry, project, tag)?,
        Command::Source {
            query,
            tag,
            version,
        } => source::exec(registry, query, tag, version)?,
    }

    Ok(())
}
