mod args;
mod config;
mod provider;
mod providers;

use args::Args;
use clap::Parser;
use config::Config;

use anyhow::Result;

use crate::{provider::LLMProvider, providers::OpenRouter};

const SYSTEM_PROMPT: &str = r#"You are a helpful assistant that provides concise, minimal responses.
When asked how to do something, provide ONLY the command or code needed, without any explanation.
Your output should be directly usable - no formatting, no explanations, no extra text.
For example, if asked "how to make a git commit", respond with only: git commit -m ""
Keep responses minimal and executable."#;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load config with proper priority: CLI args > local config > home config
    let config = Config::load(&args);

    // Combine all remaining arguments into a single string
    let combined = args.args.join(" ");

    let provider = OpenRouter::new(
        &config.api_key.expect("No API key"),
        &config.model.expect("No model"),
    );

    let response = provider.prompt(SYSTEM_PROMPT, &combined).await?;
    println!("{response}");

    Ok(())
}
