//! Sup config
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Get value from equation
fn get(src: &str, key: &str) -> String {
    if let Some(mut begin) = src.find(key) {
        begin = src[begin..].find('=').unwrap_or(begin);
        let end = src[begin..].find('\n').unwrap_or(0) + begin;
        src[begin..end].trim().to_string()
    } else {
        String::new()
    }
}

/// MetaData Config
#[derive(Deserialize, Serialize)]
struct MetaData {
    /// Node authors
    pub authors: Vec<String>,
    /// Node version
    pub version: String,
    /// Node license
    pub license: String,
}

impl Default for MetaData {
    fn default() -> MetaData {
        let git_config = String::from_utf8_lossy(
            Command::new("git")
                .args(vec!["config", "--global", "--list"])
                .output()
                .expect("Failed to execute git command")
                .stdout,
        );
        let author = get(&git_config, "user.name");
        let mail = get(&git_config, "user.mail");
        MetaData {
            authors: vec![format!("{} {}", author, mail)],
            version: "0.1".to_string(),
            license: "MIT".to_string(),
        }
    }
}

/// Node Config
struct Node {
    /// Node Registry
    pub registry: String,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            registry: "https://github.com/paritytech/substrate.git".to_string(),
        }
    }
}

/// Sup Config
pub struct Config {
    /// Node Metadata
    pub metadata: MetaData,
    /// Node Config
    pub node: Node,
}
