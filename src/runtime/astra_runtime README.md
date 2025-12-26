# Astra Runtime Crate

## Overview

The core adaptive runtime of Astra AGI, managing the event loop, tick processing, and integration with personality, emotion, memory, and planning modules.

## Main Modules & Functions

- `execute_program(input: &str)`  
  Parses and executes Astra programs or user commands.

- `tick()`  
  Advances the runtime state, processing events and reasoning.

- `run()`  
  Starts the continuous runtime loop (optional).

## Usage

Import and instantiate the runtime to process inputs and drive reasoning cycles.

```rust
use astra_runtime::Runtime;

let mut runtime = Runtime::new();

runtime.execute_program("your astra code here");
runtime.tick();
```

Documentation
Auto-generated API docs available via:

```bash
cargo doc --open
```
License
MIT OR Apache 2.0
