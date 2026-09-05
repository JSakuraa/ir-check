use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ir-check")]
#[command(about = "A small compiler diagnostics experiment for a quantum circuit DSL")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check { file: PathBuf },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file } => {
            let source = fs::read_to_string(&file)?;
            println!("{source}");
        }
    }

    Ok(())
}
