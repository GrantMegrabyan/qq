mod args;
mod config;
mod logging;
mod persona;
mod prompts;
mod provider;
mod providers;

use arboard::Clipboard;
use args::Args;
use chrono::Local;
use clap::Parser;
use config::Config;

use anyhow::Result;
use spinoff::{Color, Spinner, spinners};

use crate::logging::RequestLogEntryBuilder;
use crate::persona::Persona;
use crate::prompts::get_system_prompt;
use crate::provider::LLMProvider;
use crate::providers::OpenRouter;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let mut log_entry = RequestLogEntryBuilder::default();
    let total_start = Instant::now();
    log_entry.time(Local::now().to_rfc3339());

    let args = Args::parse();
    let config = Config::load(&args);

    let _ = run(&args, &config, &mut log_entry).await;

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

async fn run(args: &Args, config: &Config, log_entry: &mut RequestLogEntryBuilder) -> Result<()> {
    log_entry.config(config);

    // Combine all remaining arguments into a single string
    let user_prompt = args.args.join(" ");
    log_entry.user_prompt(&user_prompt);

    let provider = OpenRouter::new(&config.api_key, &config.model);

    let mut spinner = Spinner::new(
        spinners::Dots,
        format!("Asking {}", config.model),
        Color::Blue,
    );
    let persona = config.persona.clone().unwrap_or(Persona::Default);
    let system_prompt = get_system_prompt(persona);

    let llm_start = Instant::now();
    let response = provider.prompt(&system_prompt, &user_prompt).await?;
    let llm_duration = llm_start.elapsed();
    log_entry.response(&response);
    log_entry.llm_response_time_ms(llm_duration.as_millis() as u64);
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
