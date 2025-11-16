use anyhow::Result;

pub trait LLMProvider {
    async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String>;
}
