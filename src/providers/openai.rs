use anyhow::{Context, Result, anyhow};
use async_openai::{Client, config::OpenAIConfig};
use async_trait::async_trait;

use crate::provider::LLMProvider;
use crate::providers::helpers::build_openai_request;

pub struct OpenAI {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAI {
    pub fn new(api_key: &str, model: &str) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);
        Self {
            client,
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for OpenAI {
    async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let request = build_openai_request(&self.model, system_prompt, user_prompt)?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .context("Failed to get response")?;

        response
            .choices
            .first()
            .and_then(|first| first.message.content.as_ref())
            .map(|content| content.to_string())
            .ok_or_else(|| anyhow!("Response is empty or contains no content"))
    }
}
