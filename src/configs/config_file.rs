use anyhow::{Result, anyhow};
use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{configs::types::ProviderConfig, persona::Persona};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ConfigFile {
    pub provider: Option<String>,
    pub providers: Option<HashMap<String, ProviderConfig>>,
    pub persona: Option<Persona>,
    pub auto_copy: bool,
    pub log_file: Option<PathBuf>,
}

impl ConfigFile {
    pub fn update_provider(&mut self, provider_name: &str) -> Result<()> {
        // Verify provider exists in config
        if let Some(ref providers) = self.providers {
            if !providers.contains_key(provider_name) {
                let available: Vec<_> = providers.keys().map(|s| s.as_str()).collect();
                return Err(anyhow!(
                    "Provider '{}' not found in config\n\nAvailable providers: {}\nAdd a [providers.{}] section to your config",
                    provider_name,
                    available.join(", "),
                    provider_name
                ));
            }
        } else {
            return Err(anyhow!(
                "No providers configured in config\nAdd a [providers.{}] section",
                provider_name
            ));
        }

        // Update provider
        self.provider = Some(String::from(provider_name));
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
                return Err(anyhow!("Provider '{}' not found in config", provider,));
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
                return Err(anyhow!("Provider '{}' not found in config", provider));
            }
        } else {
            return Err(anyhow!("No providers configured in config"));
        }

        Ok(())
    }
}
