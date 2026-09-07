use crate::diagnostic::{Diagnostic, DiagnosticCode, Severity};
use crate::ir::{Instruction, Program};

pub fn run(program: &Program) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for item in &program.instructions {
        if let Instruction::CX { control, target } = &item.instruction {
            if control == target {
                diagnostics.push(Diagnostic {
                    code: DiagnosticCode::InvalidCxOperands,
                    severity: Severity::Error,
                    message: format!("control and target both refer to `{control}`"),
                    span: item.span.clone(),
                    help: Some("use different qubits for the CX control and target".to_string()),
                });
            }
        }
    }

    diagnostics
}
