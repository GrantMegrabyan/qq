use std::path::PathBuf;

use crate::Persona;
use crate::args::Args;
use crate::configs::config_file::ConfigFile;

use anyhow::Result;
use anyhow::anyhow;
use derive_builder::Builder;

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
    pub fn from_config_file(config_file: &ConfigFile, args: &Args) -> Result<Self> {
        // Get provider name
        let provider = config_file.provider.as_ref().ok_or_else(|| {
            anyhow!("No provider selected in config at {:?}\nSet 'provider = \"openrouter\"' in your config", "config.toml")
        })?;

        // Get providers map
        let providers = config_file.providers.as_ref().ok_or_else(|| {
            anyhow!(
                "No providers configured in config at {:?}\nAdd a [providers.{}] section",
                "config.toml",
                provider
            )
        })?;

        // Get selected provider config
        let provider_config = providers.get(provider.as_str()).ok_or_else(|| {
            let available: Vec<_> = providers.keys().map(|s| s.as_str()).collect();
            anyhow!(
                "Provider '{}' not found in config\n\nAvailable providers: {}\nCheck your config at {:?}",
                provider,
                available.join(", "),
                "config.toml"
            )
        })?.clone();

        // Check if API key is set (unless overridden by CLI args)
        if args.api_key.is_none() && provider_config.api_key.trim().is_empty() {
            return Err(anyhow!(
                "API key not set for provider '{}'\n\nSet your API key with: qq use key YOUR_API_KEY\nOr edit your config at {:?}",
                provider,
                "config.toml"
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
        config_builder.log_file(config_file.log_file.clone());

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
}
