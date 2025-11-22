use anyhow::{Result, anyhow};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

use crate::args::Args;
use crate::persona::Persona;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
}

#[derive(Deserialize, Serialize, Default, Debug)]
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

        // Create default config if it doesn't exist
        if !config_path.exists() {
            println!("Creating configuration file at {:?}...", config_path);
            Self::create_default_config(&config_path)?;
        }

        let config_file = Self::read_config(&config_path)?;

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

        // Check if API key is set (unless overridden by CLI args)
        if args.api_key.is_none() && provider_config.api_key.trim().is_empty() {
            return Err(anyhow!(
                "API key not set for provider '{}'\n\nSet your API key with: qq use key YOUR_API_KEY\nOr edit your config at {:?}",
                provider,
                config_path
            ));
        }

        // Build config with provider values
        let mut config_builder = ConfigBuilder::default();
        config_builder
            .provider(provider.clone())
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

    fn create_default_config(config_path: &PathBuf) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create config directory {:?}: {}", parent, e))?;
        }

        // Default config content
        let default_config = r#"# Persona to use
persona = "default"

# Automatically copy responses to clipboard
auto_copy = true

# Log requests to a JSONL file (optional)
log_file = "./.qq.jsonl"

# Provider to use
provider = "openrouter"

# ==========================================
# Provider-Specific Configuration
# ==========================================

# OpenRouter Provider
[providers.openrouter]
api_key = ""
model = "kwaipilot/kat-coder-pro:free"
"#;

        fs::write(config_path, default_config)
            .map_err(|e| anyhow!("Failed to write config file {:?}: {}", config_path, e))?;

        Ok(())
    }

    pub fn update_provider(provider_name: String) -> Result<()> {
        let config_path = Self::get_config_path();
        let mut config_file: ConfigFile = Self::read_config(&config_path)?;

        // Verify provider exists in config
        if let Some(ref providers) = config_file.providers {
            if !providers.contains_key(&provider_name) {
                let available: Vec<_> = providers.keys().map(|s| s.as_str()).collect();
                return Err(anyhow!(
                    "Provider '{}' not found in config\n\nAvailable providers: {}\nAdd a [providers.{}] section to your config at {:?}",
                    provider_name,
                    available.join(", "),
                    provider_name,
                    config_path
                ));
            }
        } else {
            return Err(anyhow!(
                "No providers configured in config at {:?}\nAdd a [providers.{}] section",
                config_path,
                provider_name
            ));
        }

        // Update provider
        config_file.provider = Some(provider_name.clone());

        Self::save_config(&config_file, &config_path)?;

        println!("✓ Provider set to '{}'", provider_name);
        Ok(())
    }

    pub fn update_model(model_name: String) -> Result<()> {
        let config_path = Self::get_config_path();
        let mut config_file: ConfigFile = Self::read_config(&config_path)?;

        // Get current provider
        let provider = config_file.provider.clone().ok_or_else(|| {
            anyhow!(
                "No provider selected in config at {:?}\nSet 'provider = \"openrouter\"' first",
                config_path
            )
        })?;

        // Update model for current provider
        if let Some(ref mut providers) = config_file.providers {
            if let Some(provider_config) = providers.get_mut(&provider) {
                provider_config.model = model_name.clone();
            } else {
                return Err(anyhow!(
                    "Provider '{}' not found in config\nCheck your config at {:?}",
                    provider,
                    config_path
                ));
            }
        } else {
            return Err(anyhow!(
                "No providers configured in config at {:?}",
                config_path
            ));
        }

        Self::save_config(&config_file, &config_path)?;

        println!(
            "✓ Model set to '{}' for provider '{}'",
            model_name, provider
        );
        Ok(())
    }

    pub fn update_api_key(api_key: String) -> Result<()> {
        let config_path = Self::get_config_path();
        let mut config_file: ConfigFile = Self::read_config(&config_path)?;

        // Get current provider
        let provider = config_file.provider.clone().ok_or_else(|| {
            anyhow!(
                "No provider selected in config at {:?}\nSet 'provider = \"openrouter\"' first",
                config_path
            )
        })?;

        // Update API key for current provider
        if let Some(ref mut providers) = config_file.providers {
            if let Some(provider_config) = providers.get_mut(&provider) {
                provider_config.api_key = api_key.clone();
            } else {
                return Err(anyhow!(
                    "Provider '{}' not found in config\nCheck your config at {:?}",
                    provider,
                    config_path
                ));
            }
        } else {
            return Err(anyhow!(
                "No providers configured in config at {:?}",
                config_path
            ));
        }

        Self::save_config(&config_file, &config_path)?;

        println!("✓ API key set for provider '{}'", provider);
        Ok(())
    }

    fn read_config(config_path: &PathBuf) -> Result<ConfigFile> {
        // Load config file
        fs::read_to_string(config_path)
            .map_err(|e| anyhow!("Failed to read config file {:?}: {}", config_path, e))
            .and_then(|content| {
                toml::from_str::<ConfigFile>(&content)
                    .map_err(|e| anyhow!("Failed to parse config file {:?}: {}", config_path, e))
            })
    }

    fn save_config(config_file: &ConfigFile, config_path: &PathBuf) -> Result<()> {
        let new_content = toml::to_string_pretty(config_file)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;
        fs::write(config_path, new_content)
            .map_err(|e| anyhow!("Failed to write config file {:?}: {}", config_path, e))
    }
}
