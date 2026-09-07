use std::collections::HashMap;

use crate::diagnostic::{Diagnostic, DiagnosticCode, Severity};
use crate::ir::{Instruction, Program, Span};

pub fn run(program: &Program) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut declarations: HashMap<String, Span> = HashMap::new();

    for item in &program.instructions {
        match &item.instruction {
            Instruction::QubitDeclaration { name } => {
                if declarations.contains_key(name) {
                    diagnostics.push(Diagnostic {
                        code: DiagnosticCode::DuplicateDeclaration,
                        severity: Severity::Error,
                        message: format!("Duplicate declaration of qubit `{name}`"),
                        span: item.span.clone(),
                        help: Some(format!("Remove the duplicate declaration of `{name}`")),
                    });
                } else {
                    declarations.insert(name.clone(), item.span.clone());
                }
            }

            Instruction::H { qubit }
            | Instruction::X { qubit }
            | Instruction::Measure { qubit } => {
                check_declared(qubit, &item.span, &declarations, &mut diagnostics);
            }

            Instruction::CX { control, target } => {
                check_declared(control, &item.span, &declarations, &mut diagnostics);

                check_declared(target, &item.span, &declarations, &mut diagnostics);
            }
        }
    }

    diagnostics
}

fn check_declared(
    qubit: &str,
    span: &Span,
    declarations: &HashMap<String, Span>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !declarations.contains_key(qubit) {
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::UnknownQubit,
            severity: Severity::Error,
            message: format!("unknown qubit `{qubit}`"),
            span: span.clone(),
            help: Some(format!("declare `{qubit}` before using it")),
        });
    }
}
