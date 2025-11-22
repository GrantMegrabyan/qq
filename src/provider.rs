use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider {
    async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String>;
}
