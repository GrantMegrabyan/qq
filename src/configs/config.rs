use std::path::PathBuf;

use crate::Persona;
use crate::args::Args;
use crate::configs::config_file::ConfigFile;
use crate::provider::Provider;

use anyhow::Result;
use anyhow::anyhow;
use derive_builder::Builder;

#[derive(Builder, Debug, Default)]
#[builder(setter(into))]
pub struct Config {
    pub provider: Provider,
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
                "No providers configured in config at {:?}\nAdd a [providers.{:?}] section",
                "config.toml",
                provider
            )
        })?;

        // Get selected provider config
        let provider_config = providers.get(provider).ok_or_else(|| {
            let available: Vec<_> = providers.keys().map(|p| format!("{:?}", p)).collect();
            anyhow!(
                "Provider '{:?}' not found in config\n\nAvailable providers: {}\nCheck your config at {:?}",
                provider,
                available.join(", "),
                "config.toml"
            )
        })?.clone();

        // Check if API key is set (unless overridden by CLI args)
        if args.api_key.is_none() && provider_config.api_key.trim().is_empty() {
            return Err(anyhow!(
                "API key not set for provider '{:?}'\n\nSet your API key with: qq use key YOUR_API_KEY\nOr edit your config at {:?}",
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::configs::types::ProviderConfig;
    use std::collections::HashMap;

    fn create_test_args() -> Args {
        Args {
            command: None,
            model: None,
            persona: None,
            api_key: None,
            args: vec![],
        }
    }

    fn create_test_config_file() -> ConfigFile {
        ConfigFile {
            provider: Some(Provider::OpenRouter),
            providers: Some(HashMap::from([(
                Provider::OpenRouter,
                ProviderConfig {
                    api_key: "test-api-key".to_string(),
                    model: "anthropic/claude-3.5-sonnet".to_string(),
                },
            )])),
            persona: Some(Persona::Default),
            auto_copy: false,
            log_file: None,
        }
    }

    #[test]
    fn test_from_config_file_success() {
        let config_file = create_test_config_file();
        let args = create_test_args();

        let result = Config::from_config_file(&config_file, &args);
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.provider, Provider::OpenRouter);
        assert_eq!(config.model, "anthropic/claude-3.5-sonnet");
        assert_eq!(config.api_key, "test-api-key");
        assert_eq!(config.persona, Some(Persona::Default));
        assert!(!config.auto_copy);
    }

    #[test]
    fn test_cli_args_override_model() {
        let config_file = create_test_config_file();
        let mut args = create_test_args();
        args.model = Some("gpt-4".to_string());

        let config = Config::from_config_file(&config_file, &args)
            .expect("config should be created successfully");
        assert_eq!(config.model, "gpt-4");
    }

    #[test]
    fn test_cli_args_override_api_key() {
        let config_file = create_test_config_file();
        let mut args = create_test_args();
        args.api_key = Some("override-key".to_string());

        let config = Config::from_config_file(&config_file, &args)
            .expect("config should be created successfully");
        assert_eq!(config.api_key, "override-key");
    }

    #[test]
    fn test_missing_provider_error() {
        let mut config_file = create_test_config_file();
        config_file.provider = None;
        let args = create_test_args();

        let result = Config::from_config_file(&config_file, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_api_key_error() {
        let mut config_file = create_test_config_file();
        if let Some(providers) = &mut config_file.providers
            && let Some(provider_config) = providers.get_mut(&Provider::OpenRouter)
        {
            provider_config.api_key = "".to_string();
        }
        let args = create_test_args();

        let result = Config::from_config_file(&config_file, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_api_key_but_cli_override() {
        let mut config_file = create_test_config_file();
        if let Some(providers) = &mut config_file.providers
            && let Some(provider_config) = providers.get_mut(&Provider::OpenRouter)
        {
            provider_config.api_key = "".to_string();
        }
        let mut args = create_test_args();
        args.api_key = Some("cli-key".to_string());

        let result = Config::from_config_file(&config_file, &args);
        assert!(result.is_ok());
        let config = result.expect("config should be created successfully");
        assert_eq!(config.api_key, "cli-key");
    }

    #[test]
    fn test_provider_not_in_providers_map() {
        let mut config_file = create_test_config_file();
        config_file
            .providers
            .as_mut()
            .unwrap()
            .remove(&Provider::OpenAI);
        config_file.provider = Some(Provider::OpenAI);
        let args = create_test_args();

        let result = Config::from_config_file(&config_file, &args);
        assert!(result.is_err());
    }
}
