//! Sup config
use crate::Result;
use etc::{Etc, Meta, Read, Write};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Get value from equation
fn get(src: &str, key: &str) -> String {
    if let Some(mut begin) = src.find(key) {
        begin = src[begin..].find('=').unwrap_or(0) + begin + 1;
        let end = src[begin..].find('\n').unwrap_or(0) + begin;
        src[begin..end].trim().to_string()
    } else {
        String::new()
    }
}

/// MetaData Config
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MetaData {
    /// Node authors
    pub authors: Vec<String>,
    /// Node version
    pub version: String,
    /// Node license
    pub license: String,
}

impl MetaData {
    /// Generate the metadata tuples
    pub fn tuple(&self) -> Vec<(&str, String, Option<&str>)> {
        vec![
            (
                "authors =",
                format!("authors = {:?}", self.authors),
                Some("]\n"),
            ),
            // ("version =", format!("version = \"{}\"", self.version), None),
            ("license =", format!("license = \"{}\"", self.license), None),
        ]
    }
}

impl Default for MetaData {
    fn default() -> MetaData {
        let git_config = Command::new("git")
            .args(vec!["config", "--global", "--list"])
            .output()
            .expect("Failed to execute git command")
            .stdout;
        let git_config_str = String::from_utf8_lossy(&git_config);
        let author = get(&git_config_str, "user.name");
        let mail = get(&git_config_str, "user.email");
        MetaData {
            authors: vec![format!("{} <{}>", author, mail)],
            version: "0.1.0".to_string(),
            license: "MIT".to_string(),
        }
    }
}

/// Node Config
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Node {
    /// Node Registry
    pub registry: String,
    /// Target tag
    pub tag: Option<String>,
}

impl Node {
    /// The name space of the registry node
    pub fn name_space(&self) -> String {
        let name = self.name();
        let end = self.registry.find(&name).unwrap_or(0);
        let begin = (&self.registry[..end - 1]).rfind('/').unwrap_or(0) + 1;
        self.registry[begin..end - 1].to_string()
    }

    /// Get the name of registry
    pub fn name(&self) -> String {
        let begin = self.registry.rfind('/').unwrap_or(0) + 1;
        let end = self.registry.rfind(".git").unwrap_or(0);
        if begin - 1 == end {
            "substrate"
        } else {
            &self.registry[begin..end]
        }
        .to_string()
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            registry: "https://github.com/paritytech/substrate.git".to_string(),
            tag: None,
        }
    }
}

/// Sup Config
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    /// Node Metadata
    pub metadata: MetaData,
    /// Node Config
    pub node: Node,
    /// If the config is from global path
    #[serde(skip)]
    pub global: bool,
}

impl Config {
    /// Inject default config to path
    pub fn gen_default(config: &Etc, global: bool) -> Result<Config> {
        let default = Config::default();
        config.write(toml::to_string(&default)?)?;
        Ok(default.global(global))
    }

    /// Get config from global path
    pub fn global(mut self, global: bool) -> Config {
        self.global = global;
        self
    }

    /// New config
    pub fn new() -> Result<Config> {
        if let Ok(sup) = etc::find_up("sup.toml") {
            let bytes = Etc::from(sup).read()?;
            return Ok(toml::from_slice::<Config>(&bytes)?.global(false));
        }

        let mut home = dirs::home_dir().expect("Could not find home dir");
        home.push(".sup/config.toml");

        let config = Etc::from(home);
        if config.real_path()?.exists() {
            let bytes = config.read()?;
            if let Ok(cur) = toml::from_slice::<Config>(&bytes) {
                Ok(cur)
            } else {
                Self::gen_default(&config, true)
            }
        } else {
            Self::gen_default(&config, true)
        }
    }
}
