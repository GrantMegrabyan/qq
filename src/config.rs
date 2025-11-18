use derive_builder::Builder;
use serde::Deserialize;
use std::path::PathBuf;
use std::{env, fs};

use crate::args::Args;
use crate::persona::Persona;

#[derive(Deserialize, Default, Debug)]
struct ConfigFile {
    model: Option<String>,
    persona: Option<Persona>,
    api_key: Option<String>,
    auto_copy: bool,
    log_file: Option<PathBuf>,
}

#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
pub struct Config {
    pub model: String,
    pub api_key: String,
    pub persona: Option<Persona>,
    pub auto_copy: bool,
    pub log_file: Option<PathBuf>,
}

impl Config {
    pub fn load(args: &Args) -> Self {
        let mut config_builder = ConfigBuilder::default();

        // Check if there is a config file
        if let Some(cf) = get_config_file(&PathBuf::from(".qq"), &Self::home_config_path()) {
            if let Some(model) = cf.model {
                config_builder.model(model);
            }
            if let Some(persona) = cf.persona {
                config_builder.persona(persona);
            }
            if let Some(api_key) = cf.api_key {
                config_builder.api_key(api_key);
            }
            config_builder.auto_copy(cf.auto_copy);
            config_builder.log_file(cf.log_file);
        }

        // Check cli args
        if let Some(model) = &args.model {
            config_builder.model(model);
        }
        if let Some(persona) = &args.persona {
            config_builder.persona(persona.clone());
        }
        if let Some(api_key) = &args.api_key {
            config_builder.api_key(api_key);
        }

        config_builder.build().unwrap_or_default()
    }

    fn home_config_path() -> PathBuf {
        let mut path = env::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".qq");
        path
    }
}

fn get_config_file(local_config_path: &PathBuf, home_config_path: &PathBuf) -> Option<ConfigFile> {
    let path = if local_config_path.exists() {
        log::debug!("Using local config file: {:?}", local_config_path);
        local_config_path
    } else if home_config_path.exists() {
        log::debug!("Using home config file: {:?}", home_config_path);
        home_config_path
    } else {
        log::debug!("No config file found");
        return None;
    };

    let config_file = fs::read_to_string(path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
        .unwrap_or(ConfigFile::default());

    Some(config_file)
}
