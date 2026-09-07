mod cli;
mod diagnostic;
mod ir;
mod parser;
mod passes;
mod renderer;

use clap::Parser;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use cli::{Cli, Commands};

fn main() -> ExitCode {
    match run() {
        Ok(exit_code) => exit_code,

        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file } => {
            let source = fs::read_to_string(&file)?;
            let file_name = file.display().to_string();

            match parser::parse(&source) {
                Ok(program) => {
                    let diagnostics = passes::analyze(&program);

                    if diagnostics.is_empty() {
                        println!("✓ program is valid");
                        Ok(ExitCode::SUCCESS)
                    } else {
                        for diagnostic in &diagnostics {
                            renderer::render_diagnostic(&file_name, &source, diagnostic);
                        }

                        Ok(ExitCode::from(1))
                    }
                }

                Err(diagnostic) => {
                    renderer::render_diagnostic(&file_name, &source, &diagnostic);

                    Ok(ExitCode::from(1))
                }
            }
        }

        Commands::PrintIr { file } => {
            let source = fs::read_to_string(&file)?;
            let file_name = file.display().to_string();

            match parser::parse(&source) {
                Ok(program) => {
                    renderer::render_ir(&program);
                    Ok(ExitCode::SUCCESS)
                }

                Err(diagnostic) => {
                    renderer::render_diagnostic(&file_name, &source, &diagnostic);

                    Ok(ExitCode::from(1))
                }
            }
        }
    }
}
