# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`qq` is a command-line tool for querying LLMs (Large Language Models) from the terminal. It provides a simple interface to send prompts to various LLM providers and receive responses directly in the terminal. The tool is written in Rust using async/await patterns.

## Build and Run Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the tool (development)
cargo run -- [OPTIONS] <PROMPT>

# Run tests (if any exist)
cargo test

# Check for compilation errors without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

## Architecture

### Configuration System

The application has a layered configuration system with the following precedence (highest to lowest):

1. **CLI arguments** (`-m`, `-p`, `-a` flags)
2. **Local config file** (`.qq` in current directory)
3. **Global config file** (`~/.qq` in home directory)

Config files are TOML format and can specify:
- `model`: The LLM model to use
- `persona`: System prompt persona (currently only "default")
- `api_key`: OpenRouter API key
- `auto_copy`: Boolean to auto-copy responses to clipboard
- `log_file`: Path to JSON Lines log file (e.g., `.qq.jsonl`)

Configuration loading logic is in `src/config.rs:29-59`.

### Module Organization

- **`main.rs`**: Entry point; handles CLI parsing, config loading, LLM calls, and logging
- **`args.rs`**: CLI argument definitions using clap
- **`config.rs`**: Configuration loading with precedence from CLI → local → global
- **`provider.rs`**: `LLMProvider` trait defining the interface for LLM providers
- **`providers/`**: Concrete provider implementations
  - `open_router.rs`: OpenRouter API client using the async-openai library
- **`persona.rs`**: System prompt persona enum (currently just "Default")
- **`prompts.rs`**: System prompt templates for different personas
- **`logging.rs`**: Request/response logging to JSON Lines format

### Provider Pattern

The codebase uses a trait-based provider pattern to support multiple LLM backends:

1. `LLMProvider` trait (in `provider.rs`) defines the interface
2. Concrete implementations in `providers/` module (currently only OpenRouter)
3. OpenRouter uses the `async-openai` crate, configured to point at OpenRouter's API base URL

To add a new provider:
1. Create a new file in `src/providers/`
2. Implement the `LLMProvider` trait
3. Export from `src/providers/mod.rs`
4. Update `main.rs` to instantiate the new provider

### Logging

All requests are logged to a JSON Lines file if `log_file` is configured. Each log entry includes:
- Timestamp
- Config used (model, persona, auto_copy)
- User prompt
- LLM response
- LLM response time (milliseconds)
- Total runtime (milliseconds)

Logging is built using the builder pattern (`RequestLogEntryBuilder`) and writes happen in `main.rs:39-50` after the request completes.

### Key Dependencies

- **async-openai**: OpenAI-compatible API client (used for OpenRouter)
- **tokio**: Async runtime
- **clap**: CLI argument parsing with derive macros
- **serde/serde_json**: JSON serialization for logging
- **toml**: Config file parsing
- **arboard**: Clipboard integration for auto-copy feature
- **spinoff**: Terminal spinner while waiting for responses
- **derive_builder**: Builder pattern for Config and logging structs
