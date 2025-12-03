use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, ValueEnum, Debug, PartialEq, Hash, Eq)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
#[derive(Default)]
pub enum Provider {
    #[default]
    OpenRouter = 0,
    OpenAI = 1,
}

#[async_trait]
pub trait LLMProvider {
    async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String>;
}
