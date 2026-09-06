use std::collections::HashSet;

use crate::diagnostic::{Diagnostic, DiagnosticCode, Severity};
use crate::ir::{Instruction, Program, Span};

pub fn run(program: &Program) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut measured = HashSet::new();

    for item in &program.instructions {
        match &item.instruction {
            Instruction::Measure {qubit} => {
                if measured.contains(qubit) {
                    diagnostics.push(Diagnostic {
                        code: DiagnosticCode::DuplicateMeasurement,
                        severity: Severity::Error,
                        message: format!("qubit `{qubit}` was measured more than once"),
                        span: item.span.clone(),
                        help: Some(format!(
                            "Remove the repeated measurement of `{qubit}`"
                        )),
                    });
                } else {
                    measured.insert(qubit.clone());
                }
            }

            Instruction::H {qubit} | Instruction::X { qubit } => {
                check_active(qubit, item, &measured, &mut diagnostics);
            }

            Instruction::CX { control, target } => {
                check_active(control, item, &measured, &mut diagnostics);
                check_active(target, item, &measured, &mut diagnostics);
            }

            Instruction::QubitDeclaration { .. } => {}
        }
    }

    diagnostics
}

fn check_active(
    qubit: &str,
    item: &crate::ir::SpannedInstruction,
    measured: &HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if measured.contains(qubit) {
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::OperationAfterMeasurement,
            severity: Severity::Error,
            message: format!("Operation on measured qubit `{qubit}`"),
            span: item.span.clone(),
            help: Some(format!(
                "Move this operation before `measure {qubit}`"
            )),
        });
    }
}