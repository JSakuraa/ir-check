use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ir-check")]
#[command(about = "Compiler diagnostics for a minimal quantum circuit DSL")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Check { file: PathBuf },

    PrintIr { file: PathBuf },
}
