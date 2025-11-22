use anyhow::{Result, anyhow};
use derive_builder::Builder;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

use crate::args::Args;
use crate::persona::Persona;

#[derive(Deserialize, Debug, Clone)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
}

#[derive(Deserialize, Default, Debug)]
struct ConfigFile {
    provider: Option<String>,
    providers: Option<HashMap<String, ProviderConfig>>,
    persona: Option<Persona>,
    auto_copy: bool,
    log_file: Option<PathBuf>,
}

#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
pub struct Config {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub persona: Option<Persona>,
    pub auto_copy: bool,
    pub log_file: Option<PathBuf>,
}

impl Config {
    pub fn load(args: &Args) -> Result<Self> {
        let config_path = Self::get_config_path();

        // Load config file
        let config_file = if config_path.exists() {
            log::debug!("Using config file: {:?}", config_path);
            let content = fs::read_to_string(&config_path)
                .map_err(|e| anyhow!("Failed to read config file {:?}: {}", config_path, e))?;
            toml::from_str::<ConfigFile>(&content)
                .map_err(|e| anyhow!("Failed to parse config file {:?}: {}", config_path, e))?
        } else {
            return Err(anyhow!(
                "Config file not found. Expected at: {:?}\n\nCreate a config file with:\n  provider = \"openrouter\"\n  \n  [providers.openrouter]\n  api_key = \"sk-or-v1-...\"\n  model = \"openai/gpt-4\"",
                config_path
            ));
        };

        // Get provider name
        let provider = config_file.provider.ok_or_else(|| {
            anyhow!("No provider selected in config at {:?}\nSet 'provider = \"openrouter\"' in your config", config_path)
        })?;

        // Get providers map
        let providers = config_file.providers.ok_or_else(|| {
            anyhow!(
                "No providers configured in config at {:?}\nAdd a [providers.{}] section",
                config_path,
                provider
            )
        })?;

        // Get selected provider config
        let provider_config = providers.get(&provider).ok_or_else(|| {
            let available: Vec<_> = providers.keys().map(|s| s.as_str()).collect();
            anyhow!(
                "Provider '{}' not found in config\n\nAvailable providers: {}\nCheck your config at {:?}",
                provider,
                available.join(", "),
                config_path
            )
        })?.clone();

        // Build config with provider values
        let mut config_builder = ConfigBuilder::default();
        config_builder
            .provider(provider)
            .model(provider_config.model)
            .api_key(provider_config.api_key);

        if let Some(persona) = config_file.persona {
            config_builder.persona(persona);
        }
        config_builder.auto_copy(config_file.auto_copy);
        config_builder.log_file(config_file.log_file);

        // CLI args override
        if let Some(model) = &args.model {
            config_builder.model(model);
        }
        if let Some(persona) = &args.persona {
            config_builder.persona(*persona);
        }
        if let Some(api_key) = &args.api_key {
            config_builder.api_key(api_key);
        }

        config_builder
            .build()
            .map_err(|e| anyhow!("Failed to build config: {}", e))
    }

    fn get_config_path() -> PathBuf {
        // Check QQ_HOME_PATH environment variable
        if let Ok(qq_home) = env::var("QQ_HOME_PATH") {
            let mut path = PathBuf::from(qq_home);
            path.push("config.toml");
            return path;
        }

        // Fall back to ~/.qq/config.toml
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".qq");
        path.push("config.toml");
        path
    }
}
