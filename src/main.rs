mod args;
mod config;
mod persona;
mod prompts;
mod provider;
mod providers;

use arboard::Clipboard;
use args::Args;
use clap::Parser;
use config::Config;

use anyhow::Result;
use spinoff::{Color, Spinner, spinners};

use crate::persona::Persona;
use crate::prompts::get_system_prompt;
use crate::provider::LLMProvider;
use crate::providers::OpenRouter;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load config with proper priority: CLI args > local config > home config
    let config = Config::load(&args);

    // Combine all remaining arguments into a single string
    let combined = args.args.join(" ");

    let provider = OpenRouter::new(&config.api_key, &config.model);

    let mut spinner = Spinner::new(
        spinners::Dots,
        format!("Asking {}", config.model),
        Color::Blue,
    );
    let response = provider
        .prompt(&get_system_prompt(Persona::Default), &combined)
        .await?;
    spinner.clear();

    print!("{response}");

    if config.auto_copy && copy_to_clipboard(&response) {
        print!(" \x1b[90m(copied)\x1b[0m");
    }
    println!();

    Ok(())
}

fn copy_to_clipboard(text: &str) -> bool {
    match Clipboard::new() {
        Ok(mut cb) => cb.set_text(text).is_ok(),
        Err(_) => false,
    }
}
