# IR Check

## 1. Project Summary

**IR Check** is a small Rust-based compiler frontend and developer-tooling experiment for a minimal quantum circuit language.

The project will:

* Parse a small quantum-inspired DSL.
* Convert source code into a structured intermediate representation.
* Run semantic validation passes over that IR.
* Produce clear, source-attributed compiler diagnostics.
* Expose the workflow through a simple CLI.
* Optionally expose diagnostics as structured JSON for downstream tooling.

The purpose of the project is **not** to build a quantum simulator or production compiler.

The primary goals are to demonstrate:

* Practical Rust development.
* Compiler frontend fundamentals.
* Intermediate representation design.
* Compiler-pass architecture.
* Structured error modeling.
* Developer-experience thinking.
* Familiarity with basic quantum circuit concepts.

---

# 2. Project Goals

The finished project should demonstrate that the developer can:

1. Write and organize a small Rust application.
2. Design enums and structs representing a compiler IR.
3. Parse user-facing source code into structured data.
4. Separate parsing, semantic analysis, diagnostics, and presentation.
5. Implement compiler-style validation passes.
6. Attribute errors back to source locations.
7. Design useful, actionable error messages.
8. Build a clean command-line developer workflow.
9. Write unit and integration tests.
10. Explain architectural tradeoffs clearly in a README.

---

# 3. Non-Goals

The following are explicitly outside the project scope.

Do **not** add them during the initial build:

* Quantum state-vector simulation.
* Physical qubit modeling.
* Hardware execution.
* Pulse-level compilation.
* QASM compatibility.
* Optimization passes.
* SSA implementation.
* Control flow.
* Loops.
* Functions.
* Variables beyond qubit identifiers.
* Python bindings.
* LSP implementation.
* GUI or web UI.
* Network APIs.
* Async Rust.
* Database integration.

The project succeeds by being **small, polished, understandable, and complete**.

---

# 4. User Experience

A user writes a file such as:

```text
qubit q0
qubit q1

h q0
cx q0 q1

measure q0
measure q1
```

Then runs:

```bash
ir-check check examples/bell.qir
```

Successful output:

```text
✓ program is valid
```

An invalid file:

```text
qubit q0

h q0
measure q0
x q0
measure q1
```

should produce diagnostics resembling:

```text
error[E003]: operation on measured qubit
  --> examples/invalid.qir:5:3
   |
 4 | measure q0
 5 | x q0
   |   ^^ q0 has already been measured
   |
help: move this operation before `measure q0`
```

and:

```text
error[E001]: unknown qubit `q1`
  --> examples/invalid.qir:6:9
   |
 6 | measure q1
   |         ^^ q1 was never declared
   |
help: declare it first with `qubit q1`
```

The emphasis should be on **useful feedback**, not merely determining whether the program is valid.

---

# 5. Language Design

The DSL should intentionally remain tiny.

Suggested file extension:

```text
.qir
```

This is only an internal project format and should not claim compatibility with any existing quantum IR standard.

## Supported instructions

### Qubit declaration

```text
qubit q0
```

Grammar:

```text
qubit <identifier>
```

---

### Hadamard gate

```text
h q0
```

Grammar:

```text
h <qubit>
```

---

### Pauli-X gate

```text
x q0
```

Grammar:

```text
x <qubit>
```

---

### Controlled-NOT

```text
cx q0 q1
```

Grammar:

```text
cx <control> <target>
```

---

### Measurement

```text
measure q0
```

Grammar:

```text
measure <qubit>
```

---

# 6. Basic Quantum Semantics

The language does not need to simulate quantum state.

It only models a sequence of circuit operations.

The following semantic rules should be enforced.

## Rule 1 — Qubits must be declared before use

Invalid:

```text
h q0
```

Diagnostic:

```text
E001 — Unknown Qubit
```

---

## Rule 2 — Qubit names must be unique

Invalid:

```text
qubit q0
qubit q0
```

Diagnostic:

```text
E002 — Duplicate Qubit Declaration
```

The diagnostic should reference both:

* The duplicate declaration.
* The original declaration, if practical.

---

## Rule 3 — Gates cannot operate on measured qubits

Invalid:

```text
qubit q0
measure q0
h q0
```

Diagnostic:

```text
E003 — Operation After Measurement
```

---

## Rule 4 — A qubit cannot be measured twice

Invalid:

```text
qubit q0
measure q0
measure q0
```

Diagnostic:

```text
E004 — Duplicate Measurement
```

---

## Rule 5 — CX operands must be different

Invalid:

```text
qubit q0
cx q0 q0
```

Diagnostic:

```text
E005 — Invalid CX Operands
```

Suggested explanation:

```text
control and target must refer to different qubits
```

---

# 7. Source Location Model

Every meaningful parsed instruction should retain its location in the original source.

Define a source span similar to:

```rust
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}
```

A slightly more flexible version is also acceptable:

```rust
pub struct Span {
    pub start: usize,
    pub end: usize,
}
```

If absolute offsets are used, line and column can be derived during diagnostic rendering.

The important design principle is:

> Compiler analysis operates on structured IR while preserving enough source metadata to attribute diagnostics back to the user's program.

---

# 8. Intermediate Representation

The parser should produce a `Program`.

Example:

```rust
pub struct Program {
    pub instructions: Vec<SpannedInstruction>,
}
```

Each instruction should contain:

```rust
pub struct SpannedInstruction {
    pub instruction: Instruction,
    pub span: Span,
}
```

Suggested instruction enum:

```rust
pub enum Instruction {
    QubitDecl {
        name: String,
    },

    H {
        qubit: String,
    },

    X {
        qubit: String,
    },

    CX {
        control: String,
        target: String,
    },

    Measure {
        qubit: String,
    },
}
```

The exact structure may evolve during implementation.

Prefer clear modeling over premature abstraction.

---

# 9. Compiler Pipeline

The project should follow this conceptual pipeline:

```text
Source File
    ↓
Parser
    ↓
Program IR
    ↓
Semantic Analysis Passes
    ↓
Vec<Diagnostic>
    ↓
Renderer
    ↓
Terminal / JSON
```

Each stage should have a clear responsibility.

---

# 10. Parser

The initial parser should be deliberately straightforward.

Recommended strategy:

1. Read the file as a string.
2. Iterate over lines.
3. Trim whitespace.
4. Ignore blank lines.
5. Split each line into tokens.
6. Match the first token to an instruction.
7. Validate token count.
8. Construct an `Instruction`.
9. Attach source location information.

No parser generator is necessary.

The parser should detect syntax-level issues such as:

```text
cx q0
```

or:

```text
h
```

or:

```text
banana q0
```

These should create parse diagnostics rather than panic.

Suggested parse error:

```text
E000 — Invalid Syntax
```

Example:

```text
error[E000]: `cx` expects two qubit operands
  --> program.qir:4:1
   |
 4 | cx q0
   | ^^^^^
   |
help: expected `cx <control> <target>`
```

---

# 11. Diagnostic Model

Diagnostics are a major focus of the project.

They should be represented as data rather than immediately printed.

Suggested model:

```rust
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub help: Option<String>,
}
```

Optional additions:

```rust
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message: String,
    pub primary_span: Span,
    pub secondary_spans: Vec<LabeledSpan>,
    pub help: Option<String>,
}
```

Suggested severity enum:

```rust
pub enum Severity {
    Error,
    Warning,
}
```

For the initial project, every semantic diagnostic may simply be an error.

---

# 12. Error Codes

Use stable error codes.

```text
E000 — Invalid Syntax
E001 — Unknown Qubit
E002 — Duplicate Qubit Declaration
E003 — Operation After Measurement
E004 — Duplicate Measurement
E005 — Invalid CX Operands
```

These codes should be represented by an enum rather than loose strings where practical.

Example:

```rust
pub enum DiagnosticCode {
    InvalidSyntax,
    UnknownQubit,
    DuplicateDeclaration,
    OperationAfterMeasurement,
    DuplicateMeasurement,
    InvalidCxOperands,
}
```

Rendering can convert these into `E000`, `E001`, etc.

---

# 13. Compiler Pass Architecture

Semantic analysis should not live inside the CLI.

The project should establish a small pass abstraction.

For example:

```rust
pub trait Pass {
    fn run(&self, program: &Program) -> Vec<Diagnostic>;
}
```

Potential passes:

```text
DeclarationPass
MeasurementPass
OperandValidationPass
```

An alternative architecture with functions instead of trait objects is acceptable if it is simpler.

The important design rule is:

> Semantic validation should remain independent of diagnostic rendering and CLI behavior.

---

# 14. Pass 1 — Declaration Analysis

Responsibilities:

* Track declared qubits.
* Detect duplicate declarations.
* Detect references to undeclared qubits.

Possible internal structure:

```rust
HashMap<String, Span>
```

The map stores the first declaration location for each qubit.

Example:

```text
qubit q0
qubit q0
```

should produce `E002`.

---

# 15. Pass 2 — Measurement Analysis

Track qubit state as instructions are visited.

Conceptually:

```rust
enum QubitState {
    Active,
    Measured,
}
```

The pass should detect:

* Gate after measurement.
* Repeated measurement.

Example:

```text
measure q0
h q0
```

produces `E003`.

Example:

```text
measure q0
measure q0
```

produces `E004`.

---

# 16. Pass 3 — Operand Validation

Validate instruction-level constraints.

Initially this only needs to verify:

```text
cx control target
```

has distinct operands.

Invalid:

```text
cx q0 q0
```

produces `E005`.

---

# 17. CLI

Use `clap`.

The binary should be called:

```text
ir-check
```

Minimum command:

```bash
ir-check check <FILE>
```

Example:

```bash
ir-check check examples/bell.qir
```

Exit behavior:

```text
0 = valid program
1 = compiler/validation errors
2 = CLI or filesystem failure
```

Exact codes may change, but successful versus invalid execution should be distinguishable.

---

# 18. IR Inspection Command

Add:

```bash
ir-check print-ir <FILE>
```

Example output:

```text
Program
├── Declare q0
├── Declare q1
├── H q0
├── CX q0, q1
├── Measure q0
└── Measure q1
```

This command exists primarily to demonstrate:

* IR construction.
* Compiler introspection.
* Developer tooling.

It does not need sophisticated formatting.

---

# 19. Diagnostic Rendering

Preferred library:

```text
ariadne
```

Alternative:

```text
miette
```

Either is acceptable.

The rendered diagnostic should ideally contain:

* Error severity.
* Error code.
* Human-readable message.
* Filename.
* Source line.
* Highlighted span.
* Helpful explanatory message.

Example:

```text
error[E001]: unknown qubit `q2`
  --> examples/bell-broken.qir:5:9
   |
 5 | measure q2
   |         ^^ q2 was never declared
   |
help: declare the qubit before its first use
```

Error messages should answer three questions:

1. **What happened?**
2. **Where did it happen?**
3. **What should the developer do next?**

---

# 20. Optional JSON Diagnostics

This is a stretch goal after the core project is complete.

Command:

```bash
ir-check check program.qir --format json
```

Output:

```json
{
  "valid": false,
  "diagnostics": [
    {
      "code": "E001",
      "severity": "error",
      "message": "unknown qubit `q2`",
      "line": 5,
      "column": 9,
      "help": "declare the qubit before its first use"
    }
  ]
}
```

This feature should reuse the same underlying diagnostic structures used by terminal rendering.

Do not construct separate error models for JSON and terminal output.

This demonstrates that compiler diagnostics could be consumed by:

* Python tooling.
* Notebook environments.
* LSP clients.
* IDE integrations.
* Visualization layers.

---

# 21. Recommended Dependencies

Keep dependencies limited.

Suggested:

```toml
[dependencies]
clap = { version = "...", features = ["derive"] }
thiserror = "..."
ariadne = "..."
serde = { version = "...", features = ["derive"], optional = true }
serde_json = { version = "...", optional = true }
```

Avoid adding dependencies unless they solve an actual project requirement.

---

# 22. Suggested Repository Structure

```text
ir-check/
├── Cargo.toml
├── README.md
├── examples/
│   ├── bell.qir
│   ├── ghz.qir
│   ├── unknown-qubit.qir
│   ├── duplicate-qubit.qir
│   └── measured-qubit.qir
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── parser.rs
│   ├── ir.rs
│   ├── diagnostic.rs
│   ├── renderer.rs
│   └── passes/
│       ├── mod.rs
│       ├── declarations.rs
│       ├── measurement.rs
│       └── operands.rs
└── tests/
    ├── parser.rs
    └── validation.rs
```

This structure is a target, not a requirement from the first commit.

Start simpler and split files as responsibilities become clear.

---

# 23. Example Programs

## Bell State

```text
qubit q0
qubit q1

h q0
cx q0 q1

measure q0
measure q1
```

This is the canonical valid example.

---

## GHZ-Style Circuit

```text
qubit q0
qubit q1
qubit q2

h q0
cx q0 q1
cx q1 q2

measure q0
measure q1
measure q2
```

The tool does not need to verify that this actually creates the intended quantum state.

It only validates circuit structure.

---

## Unknown Qubit

```text
qubit q0

h q0
cx q0 q1
```

Expected:

```text
E001
```

---

## Duplicate Declaration

```text
qubit q0
qubit q0
```

Expected:

```text
E002
```

---

## Operation After Measurement

```text
qubit q0

h q0
measure q0
x q0
```

Expected:

```text
E003
```

---

## Duplicate Measurement

```text
qubit q0

measure q0
measure q0
```

Expected:

```text
E004
```

---

## Invalid CX

```text
qubit q0

cx q0 q0
```

Expected:

```text
E005
```

---

# 24. Testing Requirements

The project should contain automated tests before it is considered finished.

## Parser Tests

Test:

* Qubit declaration parses.
* H parses.
* X parses.
* CX parses.
* Measure parses.
* Blank lines are ignored.
* Invalid instruction is rejected.
* Incorrect operand count is rejected.

---

## Semantic Tests

Test:

* Bell-state example is valid.
* GHZ-style example is valid.
* Unknown qubit produces `E001`.
* Duplicate declaration produces `E002`.
* Gate after measurement produces `E003`.
* Double measurement produces `E004`.
* Same-control-target CX produces `E005`.

Tests should inspect structured diagnostics where possible rather than comparing entire terminal strings.

For example:

```rust
assert_eq!(diagnostics[0].code, DiagnosticCode::UnknownQubit);
```

This keeps tests stable even if presentation changes.

---

# 25. README Requirements

The README is part of the project deliverable.

It should contain:

## Overview

Explain that IR Check is:

> A small Rust compiler-front-end experiment exploring structured diagnostics and developer ergonomics for a minimal quantum circuit DSL.

---

## Motivation

Explain that the project investigates:

* How compiler errors should be represented.
* How diagnostics remain independent from presentation.
* How semantic analysis can operate over a small IR.
* How source attribution improves the developer workflow.

---

## Example

Show:

```bash
ir-check check examples/bell.qir
```

and at least one attractive error diagnostic.

---

## Architecture

Include:

```text
Source
  ↓
Parser
  ↓
IR
  ↓
Validation Passes
  ↓
Diagnostics
  ↓
Terminal / JSON
```

---

## Design Decisions

Briefly discuss:

### Structured diagnostics

Diagnostics are returned as data instead of printed directly by compiler passes.

### Source spans

The IR retains source attribution so downstream passes can provide useful diagnostics.

### Small pass architecture

Semantic checks are isolated from parsing and presentation.

### Intentional language limitations

The DSL deliberately avoids simulation and advanced compiler concepts so the project can focus on compiler ergonomics.

---

## Future Work

Possible extensions:

* Warnings.
* More gates.
* Circuit visualizer.
* OpenQASM frontend.
* Python bindings.
* Notebook integration.
* LSP support.
* Compilation into another IR.
* Small optimizer.
* SSA-based circuit representation.

These belong in `Future Work`, not the initial implementation.

---

# 26. Definition of Done

The project is complete when all of the following are true:

* [ ] Rust project builds successfully.
* [ ] `cargo test` passes.
* [ ] `cargo clippy` passes without meaningful warnings.
* [ ] `cargo fmt --check` passes.
* [ ] `.qir` files can be parsed.
* [ ] A `Program` IR is produced.
* [ ] Undeclared qubits are detected.
* [ ] Duplicate declarations are detected.
* [ ] Operations after measurement are detected.
* [ ] Duplicate measurements are detected.
* [ ] Invalid CX operands are detected.
* [ ] Diagnostics contain error codes.
* [ ] Diagnostics contain source locations.
* [ ] Diagnostics contain actionable messages.
* [ ] `ir-check check <FILE>` works.
* [ ] `ir-check print-ir <FILE>` works.
* [ ] Bell-state example validates successfully.
* [ ] Invalid examples produce useful errors.
* [ ] README explains architecture and design decisions.
* [ ] Repository is clean enough to link directly from a resume.

Optional:

* [ ] JSON diagnostic output works.

---

# 27. Implementation Order

Follow this order rather than trying to build the complete architecture immediately.

## Phase 1 — Rust Foundation

Goal:

```text
cargo run
```

successfully invokes a basic CLI.

Tasks:

1. Create Cargo project.
2. Add `clap`.
3. Implement `check <FILE>`.
4. Read file contents.
5. Print file contents temporarily.

---

## Phase 2 — IR

Goal:

Create the Rust data structures representing the language.

Tasks:

1. Define `Program`.
2. Define `Instruction`.
3. Define `Span`.
4. Add basic unit tests.

---

## Phase 3 — Parser

Goal:

Convert source text into the IR.

Tasks:

1. Parse line by line.
2. Recognize supported instructions.
3. Validate operand counts.
4. Record source spans.
5. Return parse diagnostics instead of panicking.

---

## Phase 4 — Diagnostic Model

Goal:

Create a stable structured error representation.

Tasks:

1. Define `Diagnostic`.
2. Define `DiagnosticCode`.
3. Define `Severity`.
4. Define optional help.
5. Ensure parser errors use this structure.

---

## Phase 5 — Declaration Analysis

Goal:

Implement:

```text
E001
E002
```

Tasks:

1. Track declarations.
2. Detect duplicate declarations.
3. Detect undeclared references.

---

## Phase 6 — Measurement Analysis

Goal:

Implement:

```text
E003
E004
```

Tasks:

1. Track measured qubits.
2. Detect gates after measurement.
3. Detect repeated measurement.

---

## Phase 7 — Operand Validation

Goal:

Implement:

```text
E005
```

Tasks:

1. Validate CX control and target.
2. Return structured diagnostic.

---

## Phase 8 — Terminal Diagnostics

Goal:

Make errors pleasant to read.

Tasks:

1. Integrate diagnostic-rendering library.
2. Show source snippets.
3. Highlight spans.
4. Add help text.
5. Confirm multiple diagnostics render cleanly.

---

## Phase 9 — IR Inspection

Goal:

Implement:

```bash
ir-check print-ir examples/bell.qir
```

Keep output simple.

---

## Phase 10 — Tests and Polish

Tasks:

1. Add example programs.
2. Add parser tests.
3. Add semantic-analysis tests.
4. Run formatter.
5. Run Clippy.
6. Improve names.
7. Remove unused code.
8. Finish README.

At this point the project is resume-ready.

---

# 28. Stretch Phase — Structured Output

Only begin this after the project meets the Definition of Done.

Implement:

```bash
ir-check check program.qir --format json
```

Add `serde`.

Reuse the existing `Diagnostic` model.

The important architectural property should remain:

```text
Compiler Pass
      ↓
Diagnostic
   ↙       ↘
Terminal    JSON
Renderer    Renderer
```

---

# 29. Resume Positioning

Recommended project name:

**IR Check — Quantum Circuit Compiler Diagnostics**

Suggested resume bullet:

> Built a Rust compiler-front-end prototype that parses a minimal quantum circuit DSL into an intermediate representation and executes semantic validation passes, with structured source-attributed diagnostics, stable error codes, and actionable compiler errors.

Possible second bullet:

> Separated compiler diagnostics from presentation to support terminal and structured output, enabling future integration with Python SDKs, notebooks, or language tooling.

Do not describe the project as:

* A quantum compiler.
* A quantum simulator.
* Production quantum software.
* An implementation of QuEra technology.

The value of the project is its compiler and developer-tooling architecture.

---

# 30. Interview Narrative

The project's core design story should eventually be explainable in roughly one minute:

> I wanted a small Rust project that exposed me to compiler-tooling concepts rather than building another generic CLI. I created a minimal quantum circuit DSL because I already had some foundational quantum-computing experience. The source is parsed into a small IR, and independent semantic passes operate over that representation. The main thing I focused on was the error model: passes return structured diagnostics containing stable codes and source spans rather than printing errors themselves. That allows the same compiler output to be rendered for a CLI today and potentially consumed by Python tooling, notebooks, or an LSP later.

That is the architectural idea the project should preserve throughout implementation.
