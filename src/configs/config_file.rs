use std::{collections::HashMap, path::PathBuf};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{configs::types::ProviderConfig, persona::Persona, provider::Provider};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ConfigFile {
    pub provider: Option<Provider>,
    pub providers: Option<HashMap<Provider, ProviderConfig>>,
    pub persona: Option<Persona>,
    pub auto_copy: bool,
    pub log_file: Option<PathBuf>,
}

impl ConfigFile {
    pub fn update_provider(&mut self, provider: &Provider) -> Result<()> {
        // Verify provider exists in config
        if let Some(ref providers) = self.providers {
            if !providers.contains_key(provider) {
                let available: Vec<String> = providers.keys().map(|p| format!("{:?}", p)).collect();
                return Err(anyhow!(
                    "Provider '{:?}' not found in config\n\nAvailable providers: {}\nAdd a [providers.{:?}] section to your config",
                    provider,
                    available.join(", "),
                    provider
                ));
            }
        } else {
            return Err(anyhow!(
                "No providers configured in config\nAdd a [providers.{:?}] section",
                provider
            ));
        }

        // Update provider
        self.provider = Some(provider.clone());
        Ok(())
    }

    pub fn update_model(&mut self, model_name: &str) -> Result<()> {
        // Get current provider
        let provider = self.provider.clone().ok_or_else(|| {
            anyhow!("No provider selected in config\nSet 'provider = \"openrouter\"' first")
        })?;

        // Update model for current provider
        if let Some(ref mut providers) = self.providers {
            if let Some(provider_config) = providers.get_mut(&provider) {
                provider_config.model = String::from(model_name);
            } else {
                return Err(anyhow!("Provider '{:?}' not found in config", provider,));
            }
        } else {
            return Err(anyhow!("No providers configured in config"));
        }

        Ok(())
    }

    pub fn update_api_key(&mut self, api_key: &str) -> Result<()> {
        // Get current provider
        let provider = self.provider.clone().ok_or_else(|| {
            anyhow!("No provider selected in config\nSet 'provider = \"openrouter\"' first")
        })?;

        // Update API key for current provider
        if let Some(ref mut providers) = self.providers {
            if let Some(provider_config) = providers.get_mut(&provider) {
                provider_config.api_key = String::from(api_key);
            } else {
                return Err(anyhow!("Provider '{:?}' not found in config", provider));
            }
        } else {
            return Err(anyhow!("No providers configured in config"));
        }

        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use std::collections::HashMap;

    use anyhow::Result;

    use super::{ConfigFile, Persona, ProviderConfig};
    use crate::provider::Provider;

    fn create_config_file() -> ConfigFile {
        ConfigFile {
            providers: Some(HashMap::from([
                (
                    Provider::OpenRouter,
                    ProviderConfig {
                        api_key: "openrouter-key".to_string(),
                        model: "gpt-4".to_string(),
                    },
                ),
                (
                    Provider::OpenAI,
                    ProviderConfig {
                        api_key: "openai-key".to_string(),
                        model: "gpt-3.5".to_string(),
                    },
                ),
            ])),
            provider: Some(Provider::OpenRouter),
            persona: Some(Persona::Default),
            auto_copy: true,
            log_file: None,
        }
    }

    #[test]
    fn test_update_model() -> Result<()> {
        let mut config = create_config_file();
        config.update_model("gpt-5")?;
        assert_eq!(
            config.providers.unwrap()[&Provider::OpenRouter].model,
            "gpt-5"
        );
        Ok(())
    }

    #[test]
    fn test_update_model_no_provider_selected() -> Result<()> {
        let mut config = create_config_file();
        config.provider = None;
        let result = config.update_model("gpt-5");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_update_api_key() -> Result<()> {
        let mut config = create_config_file();
        config.update_api_key("new-api-key")?;
        assert_eq!(
            config.providers.unwrap()[&Provider::OpenRouter].api_key,
            "new-api-key"
        );
        Ok(())
    }

    #[test]
    fn test_update_provider() -> Result<()> {
        let mut config = create_config_file();
        config.update_provider(&Provider::OpenAI)?;
        assert_eq!(config.provider.unwrap(), Provider::OpenAI);
        Ok(())
    }

    #[test]
    fn test_update_provider_to_non_existing() -> Result<()> {
        let mut config = create_config_file();
        config.providers.as_mut().unwrap().remove(&Provider::OpenAI);
        let result = config.update_provider(&Provider::OpenAI);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_update_provider_when_no_providers() -> Result<()> {
        let mut config = create_config_file();
        config.providers = None;
        let result = config.update_provider(&Provider::OpenRouter);
        assert!(result.is_err());
        Ok(())
    }
}
