use clap::Parser;

#[derive(Parser)]
#[command(name = "qq")]
#[command(about = "Query LLMs from the command line")]
pub struct Args {
    /// Model to use
    #[arg(short, long)]
    pub model: Option<String>,

    /// Persona to use
    #[arg(short, long)]
    pub persona: Option<String>,

    /// Rest of the arguments to be combined into a single string
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}
