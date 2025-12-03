use clap::{Parser, Subcommand};

use crate::{persona::Persona, provider::Provider};

#[derive(Parser)]
#[command(name = "qq")]
#[command(version)]
#[command(about = "Query LLMs from the command line")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Model to use
    #[arg(short, long)]
    pub model: Option<String>,

    /// Persona to use
    #[arg(short, long)]
    pub persona: Option<Persona>,

    /// API key
    #[arg(short, long)]
    pub api_key: Option<String>,

    /// Rest of the arguments to be combined into a single string
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure provider and model settings
    #[command(name = "use")]
    Use {
        #[command(subcommand)]
        target: UseTarget,
    },
}

#[derive(Subcommand)]
pub enum UseTarget {
    /// Set the active provider
    Provider {
        /// Provider name (e.g., "openrouter")
        name: Provider,
    },
    /// Set the model for the active provider
    Model {
        /// Model name (e.g., "anthropic/claude-3.5-sonnet")
        name: String,
    },
    /// Set the API key for the active provider
    Key {
        /// API key for the current provider
        key: String,
    },
}
