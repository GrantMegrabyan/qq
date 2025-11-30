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
        Self::with_base_url(api_key, model, OPEN_ROUTER_API_BASE)
    }

    fn with_base_url(api_key: &str, model: &str, base_url: &str) -> Self {
        let headers = Self::get_headers();
        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_default();
        let config = OpenAIConfig::new()
            .with_api_base(base_url)
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
impl LLMProvider for OpenRouter {
    async fn prompt(&self, system_prompt: &str, user_prompt: &str) -> anyhow::Result<String> {
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

    #[test]
    fn test_build_request() {
        use async_openai::types::{
            ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageContent,
            ChatCompletionRequestUserMessageContent,
        };

        let provider = OpenRouter::new("test-api-key", "anthropic/claude-3.5-sonnet");
        let system_prompt = "You are a helpful assistant";
        let user_prompt = "What is 2+2?";

        let request = provider.build_request(system_prompt, user_prompt).unwrap();

        // Verify model is set correctly
        assert_eq!(request.model, "anthropic/claude-3.5-sonnet");

        // Verify messages structure
        assert_eq!(request.messages.len(), 2);

        // Verify first message is system message with correct content
        match &request.messages[0] {
            ChatCompletionRequestMessage::System(msg) => match &msg.content {
                ChatCompletionRequestSystemMessageContent::Text(text) => {
                    assert_eq!(text, system_prompt);
                }
                _ => panic!("System message content should be text"),
            },
            _ => panic!("First message should be a system message"),
        }

        // Verify second message is user message with correct content
        match &request.messages[1] {
            ChatCompletionRequestMessage::User(msg) => match &msg.content {
                ChatCompletionRequestUserMessageContent::Text(text) => {
                    assert_eq!(text, user_prompt);
                }
                _ => panic!("User message content should be text"),
            },
            _ => panic!("Second message should be a user message"),
        }
    }
}
