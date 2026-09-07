use crate::diagnostic::Diagnostic;
use crate::ir::Program;

pub mod declarations;
pub mod measurement;
pub mod operands;

pub fn analyze(program: &Program) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    diagnostics.extend(declarations::run(program));
    diagnostics.extend(measurement::run(program));
    diagnostics.extend(operands::run(program));

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::DiagnosticCode;
    use crate::parser::parse;

    #[test]
    fn bell_program_is_valid() {
        let source = "\
qubit q0
qubit q1
h q0
cx q0 q1
measure q0
measure q1";

        let program = parse(source).unwrap();
        let diagnostics = analyze(&program);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn detects_unknown_qubit() {
        let program = parse("h q0").unwrap();
        let diagnostics = analyze(&program);

        assert!(
            diagnostics
                .iter()
                .any(|d| d.code == DiagnosticCode::UnknownQubit)
        );
    }

    #[test]
    fn detects_duplicate_declaration() {
        let program = parse("qubit q0\nqubit q0").unwrap();
        let diagnostics = analyze(&program);

        assert!(
            diagnostics
                .iter()
                .any(|d| d.code == DiagnosticCode::DuplicateDeclaration)
        );
    }

    #[test]
    fn detects_operation_after_measurement() {
        let program = parse("qubit q0\nmeasure q0\nh q0").unwrap();

        let diagnostics = analyze(&program);

        assert!(
            diagnostics
                .iter()
                .any(|d| d.code == DiagnosticCode::OperationAfterMeasurement)
        );
    }

    #[test]
    fn detects_duplicate_measurement() {
        let program = parse("qubit q0\nmeasure q0\nmeasure q0").unwrap();

        let diagnostics = analyze(&program);

        assert!(
            diagnostics
                .iter()
                .any(|d| d.code == DiagnosticCode::DuplicateMeasurement)
        );
    }

    #[test]
    fn detects_invalid_cx_operands() {
        let program = parse("qubit q0\ncx q0 q0").unwrap();

        let diagnostics = analyze(&program);

        assert!(
            diagnostics
                .iter()
                .any(|d| d.code == DiagnosticCode::InvalidCxOperands)
        );
    }
}
