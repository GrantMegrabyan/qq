use clap::ValueEnum;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Persona {
    Default,
}
