use anyhow::{Context, Result, anyhow};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
};
use async_trait::async_trait;

use crate::provider::LLMProvider;

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

    fn build_request(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> anyhow::Result<async_openai::types::CreateChatCompletionRequest> {
        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?;

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(user_prompt)
            .build()?;

        let messages = vec![
            ChatCompletionRequestMessage::System(system_message),
            ChatCompletionRequestMessage::User(user_message),
        ];

        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .build()
            .context("Failed to build request")?;

        Ok(request)
    }
}

#[async_trait]
impl LLMProvider for OpenAI {
    async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let request = self.build_request(system_prompt, user_prompt)?;

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
