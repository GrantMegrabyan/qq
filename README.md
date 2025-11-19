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

`qq` supports configuration through TOML files. It looks for configuration in this order:

1. `.qq` in the current directory (project-specific config)
2. `~/.qq` in your home directory (global config)
3. Command-line arguments (highest priority)

### Example configuration file

Create a file at `~/.qq`:

```toml
# Required: Your OpenRouter API key
api_key = "your-api-key-here"

# Required: Model to use (any model from OpenRouter)
model = "openai/gpt-5-nano"

# Optional: Automatically copy responses to clipboard (default: false)
auto_copy = true

# Optional: Log requests and responses to a file
log_file = "/Users/you/.qq.jsonl"

# Optional: System prompt persona (currently only "default" is available)
persona = "default"
```

### Get an API key

1. Sign up at [OpenRouter](https://openrouter.ai/)
2. Generate an API key
3. Add it to your config file

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
{"time":"2025-01-19T10:30:00-08:00","config":{"model":"anthropic/claude-3.5-sonnet","persona":"default","auto_copy":true},"user_prompt":"how to list files","response":"ls -la","llm_response_time_ms":450,"total_runtime_ms":502}
```

This is useful for:
- Tracking your usage
- Analyzing response times
- Building a personal knowledge base
- Debugging