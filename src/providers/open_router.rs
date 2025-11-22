use anyhow::{Context, anyhow};
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

const OPEN_ROUTER_API_BASE: &str = "https://openrouter.ai/api/v1";

pub struct OpenRouter {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenRouter {
    pub fn new(api_key: &str, model: &str) -> Self {
        let config = OpenAIConfig::new()
            .with_api_base(OPEN_ROUTER_API_BASE)
            .with_api_key(api_key);
        let client = Client::with_config(config);
        Self {
            client,
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for OpenRouter {
    async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> anyhow::Result<String> {
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

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .context("Failed to get response")?;

        match response.choices.first() {
            Some(first) => match &first.message.content {
                Some(msg) => Ok(msg.to_string()),
                None => Err(anyhow!("No content in the response message")),
            },
            None => Err(anyhow!("Reponse is empty")),
        }
    }
}
