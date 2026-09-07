use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};

use crate::diagnostic::Diagnostic;
use crate::ir::{Instruction, Program};

pub fn render_diagnostic(file_name: &str, source: &str, diagnostic: &Diagnostic) {
    let mut colors = ColorGenerator::new();
    let color = colors.next();

    let start = line_column_to_offset(source, diagnostic.span.line, diagnostic.span.column);

    let end = (start + diagnostic.span.length).min(source.len());

    let mut report = Report::build(ReportKind::Error, (file_name, start..end))
        .with_code(diagnostic.code.as_str())
        .with_message(&diagnostic.message)
        .with_label(
            Label::new((file_name, start..end))
                .with_message(&diagnostic.message)
                .with_color(color),
        );

    if let Some(help) = &diagnostic.help {
        report = report.with_help(help);
    }

    report
        .finish()
        .print((file_name, Source::from(source)))
        .expect("failed to render diagnostic");
}

pub fn render_ir(program: &Program) {
    println!("Program");

    for (index, item) in program.instructions.iter().enumerate() {
        let is_last = index == program.instructions.len() - 1;
        let prefix = if is_last { "└──" } else { "├──" };

        println!("{prefix} {}", format_instruction(&item.instruction));
    }
}

fn format_instruction(instruction: &Instruction) -> String {
    match instruction {
        Instruction::QubitDeclaration { name } => {
            format!("Declare {name}")
        }

        Instruction::H { qubit } => {
            format!("H {qubit}")
        }

        Instruction::X { qubit } => {
            format!("X {qubit}")
        }

        Instruction::CX { control, target } => {
            format!("CX {control}, {target}")
        }

        Instruction::Measure { qubit } => {
            format!("Measure {qubit}")
        }
    }
}

fn line_column_to_offset(source: &str, line: usize, column: usize) -> usize {
    let mut offset = 0;

    for (index, source_line) in source.lines().enumerate() {
        if index + 1 == line {
            return offset + column.saturating_sub(1);
        }

        // +1 accounts for the newline character.
        offset += source_line.len() + 1;
    }

    offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_cx_instruction() {
        let instruction = Instruction::CX {
            control: "q0".to_string(),
            target: "q1".to_string(),
        };

        assert_eq!(format_instruction(&instruction), "CX q0, q1");
    }

    #[test]
    fn formats_measure_instruction() {
        let instruction = Instruction::Measure {
            qubit: "q0".to_string(),
        };

        assert_eq!(format_instruction(&instruction), "Measure q0");
    }

    #[test]
    fn converts_line_column_to_offset() {
        let source = "qubit q0\nh q0\n";

        assert_eq!(line_column_to_offset(source, 2, 1), 9);
    }
}
