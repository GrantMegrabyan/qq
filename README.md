# qq

A fast, minimal command-line tool for querying LLMs directly from your terminal.

## Overview

`qq` lets you ask questions to Large Language Models and get instant responses in your terminal. It's designed for developers who want quick answers to commands, code snippets, or other questions without leaving the command line.

By default, `qq` is configured to return minimal, executable responses - just the command or code you need, without explanations or formatting.

## Features

- **Fast and minimal**: Get concise answers optimized for terminal use
- **Auto-copy**: Automatically copy responses to clipboard
- **Configurable**: Use local or global config files
- **Request logging**: Track all queries and responses in JSON Lines format
- **Multiple model support**: Works with any model available on OpenRouter

## Installation

### Build from source

```bash
git clone https://github.com/yourusername/qq.git
cd qq
cargo build --release
```

The binary will be available at `target/release/qq`. You can copy it to a directory in your PATH:

```bash
cp target/release/qq /usr/local/bin/
```

## Configuration

`qq` uses a global configuration file that is **automatically created** on first run. Configuration precedence:

1. Command-line arguments (highest priority)
2. Global config file (`~/.qq/config.toml` or `$QQ_HOME_PATH/config.toml`)

### First Run

On first run, `qq` will automatically create a default config file at `~/.qq/config.toml`:

```toml
# Persona to use
persona = "default"

# Automatically copy responses to clipboard
auto_copy = true

# Log requests to a JSONL file (optional)
log_file = "./.qq.jsonl"

# Provider to use
provider = "openrouter"

# ==========================================
# Provider-Specific Configuration
# ==========================================

# OpenRouter Provider
[providers.openrouter]
api_key = ""
model = "kwaipilot/kat-coder-pro:free"
```

### Setup

1. Sign up at [OpenRouter](https://openrouter.ai/) and get your API key at [https://openrouter.ai/keys](https://openrouter.ai/keys)
2. Run `qq` once to generate the config file
3. Edit `~/.qq/config.toml` and add your API key to the `[providers.openrouter]` section

### Custom Config Location

You can set a custom config directory using the `QQ_HOME_PATH` environment variable:

```bash
export QQ_HOME_PATH=/path/to/your/config/directory
```

The config file will be at `$QQ_HOME_PATH/config.toml`.

## Usage

Basic usage:

```bash
qq how to list files in reverse order
```

With command-line options:

```bash
# Use a specific model
qq -m anthropic/claude-3.5-haiku "explain what git rebase does"

# Use a specific API key
qq -a your-api-key "how to find large files"

# Combine options
qq -m openai/gpt-4 -p default "curl POST example with json"
```

### Command-line options

- `-m, --model <MODEL>`: Specify the model to use
- `-p, --persona <PERSONA>`: Specify the persona/system prompt
- `-a, --api-key <API_KEY>`: Specify the API key

All remaining arguments are combined into the prompt.

## Examples

```bash
# Get a git command
qq how to undo last commit
# Output: git reset --soft HEAD~1

# Get a shell command
qq find all .rs files modified in last week
# Output: find . -name "*.rs" -mtime -7

# Get code
qq rust function to read a file
# Output: std::fs::read_to_string("path/to/file")

# Complex queries work too
qq "how to make a POST request with curl including headers"
```

## Request Logging

If you configure a `log_file`, all requests and responses are logged in JSON Lines format:

```json
{"time":"2025-01-19T10:30:00-08:00","config":{"provider":"openrouter","model":"anthropic/claude-3.5-sonnet","persona":"default","auto_copy":true},"user_prompt":"how to list files","response":"ls -la","llm_response_time_ms":450,"total_runtime_ms":502}
```

This is useful for:
- Tracking your usage
- Analyzing response times
- Building a personal knowledge base
- Debugging