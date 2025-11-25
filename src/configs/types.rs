use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
}

pub trait FileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn write(&self, path: &Path, contents: &str) -> Result<()>;
}

pub struct RealFileSystem;
impl FileSystem for RealFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path).context(format!("Failed to read file at {:?}", path))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path).context(format!("Failed to create dir all at {:?}", path))
    }

    fn write(&self, path: &Path, contents: &str) -> Result<()> {
        std::fs::write(path, contents).context(format!("Failed to write file at {:?}", path))
    }
}

pub trait Environment {
    fn var(&self, key: &str) -> Result<String>;
    fn home_dir(&self) -> Option<PathBuf>;
}
pub struct RealEnvironment;
impl Environment for RealEnvironment {
    fn var(&self, key: &str) -> Result<String> {
        std::env::var(key).context(format!("Failed to get environment variable {}", key))
    }
    fn home_dir(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }
}
