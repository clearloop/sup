//! Command Config
use crate::{
    registry::Registry,
    result::{Error, Result},
};
use etc::{Etc, Meta};
use std::{path::PathBuf, process::Command};
use structopt::StructOpt;

/// Command Config
#[derive(StructOpt, Debug)]
pub enum Config {
    /// Sets config field
    Set {
        /// Substrate registry
        #[structopt(short)]
        registry: String,
    },
    /// Lists the current config
    List,
    /// Edits the current config
    Edit,
}

/// Exec `config` command
pub fn exec(mut r: Registry, config: Config) -> Result<()> {
    let cur_registry = PathBuf::from(&r.dir);
    let home = Etc::from(
        cur_registry
            .parent()
            .expect("Could not find home dir of sup")
            .parent()
            .expect("Could not find home dir of sup"),
    );

    match config {
        Config::List => {
            println!("{:#?}", &r.config);
            return Ok(());
        }
        Config::Edit => {
            Command::new("vi")
                .arg(if let Some(ref path) = r.config.dir {
                    Etc::from(path).name()?
                } else {
                    home.name()?
                })
                .status()?;
        }
        Config::Set { registry } => {
            if !registry.ends_with(".git") {
                return Err(Error::Sup(format!("Wrong git url: {}", registry)));
            }

            r.config.node.registry = registry;
            r.config.flush()?;
        }
    }

    println!("ok!");
    Ok(())
}
