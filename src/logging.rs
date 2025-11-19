use anyhow::Result;
use derive_builder::Builder;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::config::Config;
use crate::persona::Persona;

#[derive(Serialize, Builder, Clone)]
#[builder(setter(into))]
pub struct RequestLogEntry {
    pub time: String,
    pub config: ConfigForLogging,
    #[builder(default)]
    pub user_prompt: String,
    #[builder(default)]
    pub response: String,
    #[builder(default)]
    pub error: String,
    #[builder(default)]
    pub llm_response_time_ms: u64,
    pub total_runtime_ms: u64,
}

impl RequestLogEntry {
    pub fn write_to_file(&self, log_file: &PathBuf) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;
        let json = serde_json::to_string(self)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }
}

#[derive(Serialize, Clone)]
pub struct ConfigForLogging {
    pub model: String,
    pub persona: Option<Persona>,
    pub auto_copy: bool,
}

impl From<&Config> for ConfigForLogging {
    fn from(config: &Config) -> Self {
        Self {
            model: config.model.clone(),
            persona: config.persona.clone(),
            auto_copy: config.auto_copy,
        }
    }
}
