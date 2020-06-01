//! Substrate registry
mod node_template;
mod tags;

/// Exports registry
pub use self::{node_template::NODE_TEMPLATE, tags::SUBSTRATE_TAGS};
