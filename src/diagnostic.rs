use crate::ir::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub help: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    InvalidSyntax,
    UnknownQubit,
    DuplicateDeclaration,
    OperationAfterMeasurement,
    DuplicateMeasurement,
    InvalidCxOperands,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticCode::InvalidSyntax => "E000",
            DiagnosticCode::UnknownQubit => "E001",
            DiagnosticCode::DuplicateDeclaration => "E002",
            DiagnosticCode::OperationAfterMeasurement => "E003",
            DiagnosticCode::DuplicateMeasurement => "E004",
            DiagnosticCode::InvalidCxOperands => "E005",
        }
    }
}
