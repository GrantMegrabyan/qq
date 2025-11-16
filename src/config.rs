use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};

use crate::args::Args;

#[derive(Deserialize, Default)]
struct ConfigFile {
    model: Option<String>,
    persona: Option<String>,
    api_key: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub model: Option<String>,
    pub persona: Option<String>,
    pub api_key: Option<String>,
}

impl Config {
    pub fn load(args: &Args) -> Self {
        // Load home config (~/.qq)
        let home_config = Self::load_config_file(&Self::home_config_path());

        // Load local config (./.qq)
        let local_config = Self::load_config_file(&PathBuf::from(".qq"));

        // Priority: CLI args > local config > home config
        Config {
            model: args
                .model
                .clone()
                .or(local_config.model)
                .or(home_config.model),
            persona: args
                .persona
                .clone()
                .or(local_config.persona)
                .or(home_config.persona),
            api_key: std::env::var("OPENAI_API_KEY")
                .ok()
                .or(local_config.api_key)
                .or(home_config.api_key),
        }
    }

    fn home_config_path() -> PathBuf {
        let mut path = env::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".qq");
        path
    }

    fn load_config_file(path: &PathBuf) -> ConfigFile {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }
}
