//! Substrate package generator
#![warn(missing_docs)]
pub mod cmd;
mod registry;
mod result;

pub use self::{
    registry::Registry,
    result::{Error, Result},
};
