use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub players: Vec<String>,
    pub scroll_text: bool,
    pub images: bool,
    pub dynamic: Vec<String>,
    pub special_commands: SpecialCommands
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SpecialCommands {
    #[serde(default)]
    pub play_pause: HashMap<String, String>,
    #[serde(default)]
    pub next: HashMap<String, String>,
    #[serde(default)]
    pub previous: HashMap<String, String>,
}

pub fn load_config(path: &Path) -> Option<Config> {
    let Ok(contents) = fs::read_to_string(path) else {
        eprintln!("could not read config file");
        return None;
    };
    
    let Ok(config) = serde_json::from_str(&contents) else {
        eprintln!("could not parse config file");
        return None;
    };
    
    Some(config)
}
