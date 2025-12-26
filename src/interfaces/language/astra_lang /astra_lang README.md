# Astra Language Core Crate

## Overview

Defines Astra programming language core: syntax, parsing, and execution semantics.

## Main Modules & Functions

- `parse_program(source: &str) -> Program`  
  Parse Astra source code.

- `execute_program(program: &Program) -> ExecutionResult`  
  Execute parsed programs.

- `validate_syntax(source: &str) -> ValidationResult`  
  Validate program syntax.

## Usage

Foundation for runtime programs and user commands.

## Documentation

```bash
cargo doc --open
```
