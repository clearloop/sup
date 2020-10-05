//! # Substrate package manager
//!
//! [![sup](https://github.com/clearloop/sup/workflows/sup/badge.svg)](https://github.com/clearloop/sup)
//! [![crate](https://img.shields.io/crates/v/sup.svg)](https://crates.io/crates/sup)
//! [![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/sup/)
//! [![downloads](https://img.shields.io/crates/d/sup.svg)](https://crates.io/crates/sup)
//! [![LICENSE](https://img.shields.io/crates/l/sup.svg)](https://choosealicense.com/licenses/mit/)
//!
//! Master your substrate
//!
//! ## Install
//!
//! ```bash
//! cargo install sup
//! ```
//!
//! ## Usage
//!
//! ```text
//! sup 0.1.10
//!
//!     USAGE:
//! sup <SUBCOMMAND>
//!
//!     FLAGS:
//! -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//!     SUBCOMMANDS:
//! help       Prints this message or the help of the given subcommand(s)
//!     new        Create a new substrate node template
//!     source     List Source
//!     tag        List available tags
//!     update     Update registry
//!     upgrade    Upgrade substrate project
//! ```
//!
//! [substrate]: https://github.com/paritytech/substrate
#![warn(missing_docs)]
pub mod cmd;
mod registry;
mod result;

pub use self::{
    registry::Registry,
    result::{Error, Result},
};
