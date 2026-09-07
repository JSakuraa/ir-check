use crate::diagnostic::{Diagnostic, DiagnosticCode, Severity};
use crate::ir::{Instruction, Program, Span, SpannedInstruction};

pub fn parse(source: &str) -> Result<Program, Diagnostic> {
    let mut instructions = Vec::new();

    for (line_index, raw_line) in source.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        let instruction = parse_instruction(&tokens, line_number, line)?;

        instructions.push(SpannedInstruction {
            instruction,
            span: Span {
                line: line_number,
                column: 1,
                length: line.len(),
            },
        });
    }

    Ok(Program { instructions })
}

fn parse_instruction(
    tokens: &[&str],
    line: usize,
    source_line: &str,
) -> Result<Instruction, Diagnostic> {
    match tokens {
        ["qubit", name] => Ok(Instruction::QubitDeclaration {
            name: (*name).to_string(),
        }),
        ["h", qubit] => Ok(Instruction::H {
            qubit: (*qubit).to_string(),
        }),
        ["x", qubit] => Ok(Instruction::X {
            qubit: (*qubit).to_string(),
        }),
        ["cx", control, target] => Ok(Instruction::CX {
            control: (*control).to_string(),
            target: (*target).to_string(),
        }),
        ["measure", qubit] => Ok(Instruction::Measure {
            qubit: (*qubit).to_string(),
        }),

        ["cx", ..] => Err(syntax_error(
            line,
            source_line,
            "`cx` expects two qubit operands",
            Some("expected `cx <control> <target>`"),
        )),

        ["h", ..] => Err(syntax_error(
            line,
            source_line,
            "`h` expects one qubit operand",
            Some("expected `h <qubit>`"),
        )),

        ["x", ..] => Err(syntax_error(
            line,
            source_line,
            "`x` expects one qubit operand",
            Some("expected `x <qubit>`"),
        )),

        ["measure", ..] => Err(syntax_error(
            line,
            source_line,
            "`measure` expects one qubit operand",
            Some("expected `measure <qubit>`"),
        )),

        ["qubit", ..] => Err(syntax_error(
            line,
            source_line,
            "`qubit` expects one identifier",
            Some("expected `qubit <name>`"),
        )),

        [] => unreachable!(),

        _ => Err(syntax_error(line, source_line, "unknown instruction", None)),
    }
}

fn syntax_error(line: usize, source_line: &str, message: &str, help: Option<&str>) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::InvalidSyntax,
        severity: Severity::Error,
        message: message.to_string(),
        span: Span {
            line,
            column: 1,
            length: source_line.len(),
        },
        help: help.map(str::to_string),
    }
}
// Unit tests for the parser

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Instruction;

    #[test]
    fn parses_qubit_declaration() {
        let program = parse("qubit q0").unwrap();

        assert_eq!(
            program.instructions[0].instruction,
            Instruction::QubitDeclaration {
                name: "q0".to_string()
            }
        );
    }

    #[test]
    fn parses_h_gate() {
        let program = parse("h q0").unwrap();

        assert_eq!(
            program.instructions[0].instruction,
            Instruction::H {
                qubit: "q0".to_string()
            }
        );
    }

    #[test]
    fn parses_cx_gate() {
        let program = parse("cx q0 q1").unwrap();

        assert_eq!(
            program.instructions[0].instruction,
            Instruction::CX {
                control: "q0".to_string(),
                target: "q1".to_string(),
            }
        );
    }

    #[test]
    fn ignores_blank_lines() {
        let program = parse("qubit q0\n\nh q0").unwrap();

        assert_eq!(program.instructions.len(), 2);
    }

    #[test]
    fn rejects_invalid_instruction() {
        let diagnostic = parse("banana q0").unwrap_err();

        assert_eq!(diagnostic.code, DiagnosticCode::InvalidSyntax);
        assert_eq!(diagnostic.span.line, 1);
    }

    #[test]
    fn rejects_wrong_operand_count() {
        let diagnostic = parse("cx q0").unwrap_err();

        assert_eq!(diagnostic.code, DiagnosticCode::InvalidSyntax);
        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.span.line, 1);
        assert!(diagnostic.help.is_some());
    }
}
