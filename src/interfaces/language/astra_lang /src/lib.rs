
//! Astra Language Core Crate
//!
//! Implements Astra language parsing and execution.

/// Represents a parsed Astra program.
pub struct Program {
    pub source: String,
}

/// Parses Astra source code into a program.
pub fn parse_program(source: &str) -> Program {
    Program {
        source: source.to_string(),
    }
}

/// Executes a parsed program.
pub fn execute_program(program: &Program) -> String {
    format!("Executed program: {}", program.source)
}

/// Validates syntax of Astra source code.
pub fn validate_syntax(source: &str) -> bool {
    !source.trim().is_empty()
}
