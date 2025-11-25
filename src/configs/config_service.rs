use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};

use crate::Args;
use crate::configs::Config;
use crate::configs::config_file::ConfigFile;
use crate::configs::types::{Environment, FileSystem, RealEnvironment, RealFileSystem};

const DEFAULT_CONFIG: &str = r#"
# Persona to use
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

pub struct ConfigService<F: FileSystem, E: Environment> {
    fs: F,
    env: E,
}

impl<F: FileSystem, E: Environment> ConfigService<F, E> {
    pub fn new(fs: F, env: E) -> Self {
        Self { fs, env }
    }

    pub fn load(&self, args: &Args) -> Result<Config> {
        let config_path = self.get_config_path();

        if !self.fs.exists(&config_path) {
            self.create_default_config_file(&config_path)?;
        }
        let config_file = self.read_config_file(&config_path)?;
        Config::from_config_file(&config_file, args)
    }

    pub fn update_provider(&self, provider_name: &str) -> Result<()> {
        let config_path = self.get_config_path();
        let mut config_file = self.read_config_file(&config_path)?;

        config_file
            .update_provider(provider_name)
            .context(format!("Config file: {:?}", config_path))?;
        self.save_config_file(&config_file, &config_path)?;

        println!("✓ Provider set to '{}'", provider_name);
        Ok(())
    }

    pub fn update_model(&self, model_name: &str) -> Result<()> {
        let config_path = self.get_config_path();
        let mut config_file = self.read_config_file(&config_path)?;

        config_file
            .update_model(model_name)
            .context(format!("Config file: {:?}", config_path))?;
        self.save_config_file(&config_file, &config_path)?;

        println!("✓ Model set to '{}'", model_name);
        Ok(())
    }

    pub fn update_api_key(&self, api_key: &str) -> Result<()> {
        let config_path = self.get_config_path();
        let mut config_file = self.read_config_file(&config_path)?;

        config_file
            .update_api_key(api_key)
            .context(format!("Config file: {:?}", config_path))?;
        self.save_config_file(&config_file, &config_path)?;

        println!("✓ API key set");
        Ok(())
    }

    fn get_config_path(&self) -> PathBuf {
        if let Ok(qq_home) = self.env.var("QQ_HOME_PATH") {
            let mut path = PathBuf::from(qq_home);
            path.push("config.toml");
            return path;
        }

        // Fall back to ~/.qq/config.toml
        let mut path = self.env.home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".qq");
        path.push("config.toml");
        path
    }

    fn create_default_config_file(&self, config_path: &Path) -> Result<()> {
        if let Some(parent) = config_path.parent() {
            self.fs.create_dir_all(parent)?;
        }

        // todo: Create default ConfigFile and save it using `self.save_config_file()`
        self.fs
            .write(config_path, DEFAULT_CONFIG)
            .context("Failed to write default config file")
    }

    fn read_config_file(&self, config_path: &Path) -> Result<ConfigFile> {
        self.fs
            .read_to_string(config_path)
            .context("Failed to read config file")
            .and_then(|contents| {
                toml::from_str::<ConfigFile>(&contents).context("Failed to parse config file")
            })
    }

    fn save_config_file(&self, config_file: &ConfigFile, config_path: &Path) -> Result<()> {
        let new_content = toml::to_string_pretty(config_file)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;
        self.fs
            .write(config_path, &new_content)
            .map_err(|e| anyhow!("Failed to write config file {:?}: {}", config_path, e))
    }
}

pub type ProdConfigService = ConfigService<RealFileSystem, RealEnvironment>;
impl Default for ProdConfigService {
    fn default() -> Self {
        Self::new(RealFileSystem, RealEnvironment)
    }
}
