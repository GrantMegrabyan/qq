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

1. **CLI arguments** (`-m`, `-p`, `-a` flags) - highest priority
2. **Global config file** (`~/.qq/config.toml` or `$QQ_HOME_PATH/config.toml`)

Config files are TOML format with multi-provider support:
- `provider`: The active LLM provider (e.g., "openrouter")
- `persona`: System prompt persona (currently only "default")
- `auto_copy`: Boolean to auto-copy responses to clipboard
- `log_file`: Path to JSON Lines log file (e.g., `.qq.jsonl`)
- `[providers.*]`: Provider-specific configuration sections containing:
  - `api_key`: API key for the provider
  - `model`: The LLM model to use

**Environment Variables**:
- `QQ_HOME_PATH`: Custom location for the `.qq` config directory (defaults to `~/.qq`)

**Auto-creation**: If no config file exists, `qq` will automatically create a default config template at `~/.qq/config.toml` on first run.

### Module Organization

- **`main.rs`**: Entry point; handles CLI parsing, config loading, dynamic provider instantiation, LLM calls, and logging
- **`args.rs`**: CLI argument definitions using clap
- **`config.rs`**: Multi-provider configuration loading with auto-creation, supports CLI argument overrides
- **`provider.rs`**: `LLMProvider` trait with `async_trait` for dyn compatibility
- **`providers/`**: Concrete provider implementations
  - `open_router.rs`: OpenRouter API client using async-openai with custom headers for analytics
- **`persona.rs`**: System prompt persona enum (currently just "Default")
- **`prompts.rs`**: System prompt templates for different personas
- **`logging.rs`**: Request/response logging to JSON Lines format (includes provider info)

### Provider Pattern

The codebase uses a trait-based provider pattern to support multiple LLM backends:

1. `LLMProvider` trait (in `provider.rs`) defines the async interface using `async_trait`
2. Concrete implementations in `providers/` module (currently only OpenRouter)
3. Dynamic provider instantiation in `main.rs` based on config (`Box<dyn LLMProvider>`)
4. OpenRouter uses the `async-openai` crate with custom reqwest client for analytics headers

To add a new provider:
1. Create a new file in `src/providers/`
2. Implement the `LLMProvider` trait with `#[async_trait]`
3. Export from `src/providers/mod.rs`
4. Add a new match arm in `main.rs:65-74` for dynamic instantiation
5. Document the provider config in the default config template (`config.rs:136-156`)

### Logging

All requests are logged to a JSON Lines file if `log_file` is configured. Each log entry includes:
- Timestamp
- Config used (provider, model, persona, auto_copy)
- User prompt
- LLM response
- LLM response time (milliseconds)
- Total runtime (milliseconds)

Logging is built using the builder pattern (`RequestLogEntryBuilder`) and writes happen in `main.rs` after the request completes.

### Key Dependencies

- **async-openai**: OpenAI-compatible API client (used for OpenRouter)
- **async-trait**: Enables dyn-compatible async trait methods for `LLMProvider`
- **tokio**: Async runtime
- **clap**: CLI argument parsing with derive macros
- **serde/serde_json**: JSON serialization for logging
- **toml**: Config file parsing
- **dirs**: Cross-platform home directory access
- **reqwest**: HTTP client for custom headers (OpenRouter analytics)
- **arboard**: Clipboard integration for auto-copy feature
- **spinoff**: Terminal spinner while waiting for responses
- **derive_builder**: Builder pattern for Config and logging structs
- **anyhow**: Error handling with context
