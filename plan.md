## Config with multiple providers

```toml
provider = "openrouter"

# OpenRouter Provider
[providers.openrouter]
api_key = "sk-or-v1-..."
model = "kwaipilot/kat-coder-pro:free"

# OpenAI Provider
[providers.openai]
api_key = "sk-..."
model = "gpt-4"
```

## Create config on the first run if doesn't exist


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

## Check if api_key is set

During execution we should check if api_key is set, if not, show a hint
```sh
No api key for {provider} found. Set the key using:
qq key set {provider}
```

## Set api key for provider

User should be able to type this command to set or update api key in the config file
```sh
qq key set {provider} {key}
```

## Change provider

```sh
qq provider use {provider}
```

This should change the `provider` setting in the config. 
