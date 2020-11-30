//! # Substrate package manager
//!
//! <img align="right" width="400" src="https://raw.githubusercontent.com/w3f/General-Grants-Program/79ea44570c6b8f10b286817a3d1a768d29810065/src/badge_black.svg"/>
//!
//! [![etc](https://github.com/clearloop/sup/workflows/sup/badge.svg)](https://github.com/clearloop/sup)
//! [![crate](https://img.shields.io/crates/v/sup.svg)](https://crates.io/crates/sup)
//! [![downloads](https://img.shields.io/crates/d/sup.svg)](https://crates.io/crates/sup)
//! [![LICENSE](https://img.shields.io/crates/l/sup.svg)](https://choosealicense.com/licenses/apache-2.0/)
//!
//! Sup is a package manager of [substrate][substrate] which uses the github as registry.
//!
//! With `sup`, you can `new` you substrate node just in one command, `update` your substrate
//! dependencies by tags in one command as well.
//!
//! For the official tutorial of creating [substrate][substrate] node without sup, please check
//! out [Create Your First Substrate Chain][create-your-first-substrate-chain]. For the example
//! of using sup, please checkout the [GUIDE.md](./GUIDE.md).
//!
//!
//! ## Installation
//!
//! ```bash
//! cargo install sup
//! ```
//!
//! ## Usage
//!
//! ```bash
//! sup 0.2.8
//!
//!     USAGE:
//! sup [FLAGS] <SUBCOMMAND>
//!
//!     FLAGS:
//! -h, --help       Prints help information
//!     -u, --update     Updates the global registry
//!     -V, --version    Prints version information
//!
//!     SUBCOMMANDS:
//! config    Shows or edits the current config
//!     help      Prints this message or the help of the given subcommand(s)
//!     list      List registry source or tags
//!     new       Create a new substrate node template
//!     update    Update the target substrate project
//! ```
//!
//! ## Linked Projects
//!
//! Welcome to [add][pr] your projects below if you've got a adventure of the substrate ecology
//! with sup!
//!
//! + [clearloop/cydonia][cydonia]
//!
//! Sup has been recorded in [substrate-developer-hub/awesome-substrate][awesome], and granted by
//! the [web3 foundation grants program][w3f], welcome to check the awesome repo and the grants info in w3f.
//!
//! ## LICENSE
//!
//! Apache-2.0
//!
//! [awesome]: https://github.com/substrate-developer-hub/awesome-substrate#ecosystem-tools
//! [cydonia]: https://github.com/clearloop/sup/tree/w3f
//! [substrate]: https://github.com/paritytech/substrate
//! [pr]: https://github.com/clearloop/sup/pulls
//! [create-your-first-substrate-chain]: https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/
//! [w3f]: https://github.com/w3f/Open-Grants-Program
#![warn(missing_docs)]
pub mod cmd;
mod config;
mod registry;
mod result;

pub use self::{
    config::Config,
    registry::Registry,
    result::{Error, Result},
};
