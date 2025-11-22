use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, ValueEnum, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Persona {
    Default,
}
