use std::process::Command;

/// Substrate registry
pub struct Registry(pub String);

impl Registry {
    /// New registry
    pub fn new() -> Registry {
        println!("Check substrate status...");
        println!(">{}", "-".repeat(42));
        let mut substrate = dirs::home_dir().expect("Could not find home directory");
        substrate.push(".substrate");

        let registry = substrate.to_string_lossy().to_owned();
        let mut git = Command::new("git");
        if substrate.exists() {
            git.args(vec!["-C", &registry, "pull", "origin", "master"])
                .status()
                .expect(&format!("Exec git pull failed in {}", &registry));
        } else {
            git.args(vec![
                "clone",
                "https://github.com/paritytech/substrate.git",
                &registry,
            ])
            .status()
            .expect(&format!("Exec git clone failed in {}", &registry));
        }

        println!("{}<", "-".repeat(42));
        Registry(registry.to_string())
    }

    /// list substrate tags
    pub fn list_tags(&self) -> Vec<String> {
        String::from_utf8_lossy(
            &Command::new("git")
                .args(vec!["-C", &self.0, "tag", "--list"])
                .output()
                .unwrap()
                .stdout,
        )
        .to_string()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|t| t.starts_with('v'))
        .map(|t| t.to_string())
        .collect()
    }
}
