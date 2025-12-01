use anyhow::{Context, Result};
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest,
    CreateChatCompletionRequestArgs,
};

pub(crate) fn build_openai_request(
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<CreateChatCompletionRequest> {
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
        .model(model)
        .messages(messages)
        .build()
        .context("Failed to build request")?;

    Ok(request)
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use async_openai::types::{
        ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessageContent,
    };

    #[test]
    fn test_build_openai_request() {
        let model = "gpt-3.5-turbo";
        let system_prompt = "You are a helpful assistant";
        let user_prompt = "What is 2+2?";

        let request = build_openai_request(model, system_prompt, user_prompt).unwrap();

        // Verify model is set correctly
        assert_eq!(request.model, "gpt-3.5-turbo");

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
