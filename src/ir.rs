#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub instructions: Vec<SpannedInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedInstruction {
    pub instruction: Instruction,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    QubitDeclaration { name: String },
    H { qubit: String },
    X { qubit: String },
    CX { control: String, target: String },
    Measure { qubit: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_program() {
        let program = Program {
            instructions: vec![SpannedInstruction {
                instruction: Instruction::H {
                    qubit: "q0".to_string(),
                },
                span: Span {
                    line: 1,
                    column: 1,
                    length: 4,
                },
            }],
        };

        assert_eq!(program.instructions.len(), 1);
    }
}