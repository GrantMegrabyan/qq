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
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::provider::LLMProvider;

const OPEN_ROUTER_API_BASE: &str = "https://openrouter.ai/api/v1";

pub struct OpenRouter {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenRouter {
    pub fn new(api_key: &str, model: &str) -> Self {
        let headers = Self::get_headers();
        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();
        let config = OpenAIConfig::new()
            .with_api_base(OPEN_ROUTER_API_BASE)
            .with_api_key(api_key);
        let client = Client::with_config(config).with_http_client(http_client);
        Self {
            client,
            model: model.to_string(),
        }
    }

    fn get_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("http-referer"),
            HeaderValue::from_static("https://github.com/grantmegrabyan/qq"),
        );
        headers.insert(
            HeaderName::from_static("x-title"),
            HeaderValue::from_static("qq"),
        );
        headers
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

        response
            .choices
            .first()
            .and_then(|first| first.message.content.as_ref())
            .map(|content| content.to_string())
            .ok_or_else(|| anyhow!("Response is empty or contains no content"))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::OpenRouter;

    #[test]
    fn test_get_headers() {
        let headers = OpenRouter::get_headers();
        assert_eq!(headers.len(), 2);
        assert_eq!(
            headers.get("http-referer").unwrap(),
            "https://github.com/grantmegrabyan/qq"
        );
        assert_eq!(headers.get("x-title").unwrap(), "qq");
    }
}
