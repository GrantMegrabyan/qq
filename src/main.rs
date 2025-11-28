#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod args;
mod configs;
mod logging;
mod persona;
mod prompts;
mod provider;
mod providers;

use arboard::Clipboard;
use args::{Args, Commands, UseTarget};
use chrono::Local;
use clap::Parser;
use spinoff::{Color, Spinner, spinners};
use std::time::Instant;

use crate::configs::Config;
use crate::configs::ProdConfigService;
use crate::logging::RequestLogEntryBuilder;
use crate::persona::Persona;
use crate::prompts::get_system_prompt;
use crate::provider::LLMProvider;
use crate::providers::OpenRouter;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config_service = ProdConfigService::default();
    let config = match config_service.load(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error loading config: {}", err);
            std::process::exit(1);
        }
    };

    // Handle use command
    if let Some(command) = &args.command {
        match command {
            Commands::Use { target } => {
                let result = match target {
                    UseTarget::Provider { name } => config_service.update_provider(name),
                    UseTarget::Model { name } => config_service.update_model(name),
                    UseTarget::Key { key } => config_service.update_api_key(key),
                };

                if let Err(err) = result {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
                return;
            }
        }
    }

    // Normal query mode
    let mut log_entry = RequestLogEntryBuilder::default();
    let total_start = Instant::now();
    log_entry.time(Local::now().to_rfc3339());

    run(&args, &config, &mut log_entry).await;

    let total_duration = total_start.elapsed();
    log_entry.total_runtime_ms(total_duration.as_millis() as u64);

    match log_entry.build() {
        Ok(log) => {
            if let Some(log_file) = config.log_file
                && let Err(err) = log.write_to_file(&log_file)
            {
                eprintln!("{}", err);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

async fn run(args: &Args, config: &Config, log_entry: &mut RequestLogEntryBuilder) {
    log_entry.config(config);

    // Combine all remaining arguments into a single string
    let user_prompt = args.args.join(" ");
    log_entry.user_prompt(&user_prompt);

    // Dynamically instantiate provider based on config
    let provider: Box<dyn LLMProvider> = match config.provider.as_str() {
        "openrouter" => Box::new(OpenRouter::new(&config.api_key, &config.model)),
        _ => {
            let error = format!(
                "Error: Unsupported provider '{}'\n\nCurrently supported providers: openrouter",
                config.provider
            );
            log_entry.error(&error);
            eprintln!("{}", error);
            return;
        }
    };

    let mut spinner = Spinner::new(
        spinners::Dots,
        format!("Asking {}", config.model),
        Color::Blue,
    );
    let persona = config.persona.unwrap_or(Persona::Default);
    let system_prompt = get_system_prompt(persona);

    let llm_start = Instant::now();
    match provider.prompt(&system_prompt, &user_prompt).await {
        Ok(response) => {
            let llm_duration = llm_start.elapsed();
            log_entry.response(&response);
            log_entry.llm_response_time_ms(llm_duration.as_millis() as u64);
            spinner.clear();

            print!("{response}");

            if config.auto_copy && copy_to_clipboard(&response) {
                print!(" \x1b[90m(copied)\x1b[0m");
            }
            println!();
        }
        Err(err) => {
            let llm_duration = llm_start.elapsed();
            log_entry.error(format!("{:?}", err));
            log_entry.llm_response_time_ms(llm_duration.as_millis() as u64);
            spinner.clear();

            println!("{err:?}");
        }
    }
}

fn copy_to_clipboard(text: &str) -> bool {
    match Clipboard::new() {
        Ok(mut cb) => cb.set_text(text).is_ok(),
        Err(_) => false,
    }
}
