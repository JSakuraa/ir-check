mod diagnostic;
mod ir;
mod parser;

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
            match parser::parse(&source) {
                Ok(program) => {
                    println!("{program:#?}");
                }
                Err(diagnostic) => {
                    eprintln!(
                        "error[{}] on line {}: {}",
                        diagnostic.code.as_str(),
                        diagnostic.span.line,
                        diagnostic.message
                    );

                    if let Some(help) = diagnostic.help {
                        eprintln!("help: {help}");
                    }
                }
            }
        }
    }

    Ok(())
}
