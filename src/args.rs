use clap::{Parser, Subcommand};

use crate::persona::Persona;

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
    #[command(name = "use")]
    Use {
        #[command(subcommand)]
        target: UseTarget,
    },
}

#[derive(Subcommand)]
pub enum UseTarget {
    Provider { name: String },
    Model { name: String },
}
