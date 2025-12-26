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


---

### `src/runtime/src/lib.rs`

```rust
//! Astra Runtime Crate
//!
//! This crate implements the core adaptive runtime of Astra AGI,
//! including event loops, tick processing, and module integration.

pub mod personality;
pub mod emotion;
pub mod memory;
pub mod planning;

use anyhow::Result;

/// Main runtime struct managing AGI state.
pub struct Runtime {
    // internal state fields here
}

impl Runtime {
    /// Creates a new runtime instance.
    pub fn new() -> Self {
        Self {
            // initialize fields
        }
    }

    /// Executes Astra code or user input as a program.
    pub fn execute_program(&mut self, input: &str) -> Result<()> {
        // parse and execute logic here
        Ok(())
    }

    /// Advances the runtime state by one tick.
    pub fn tick(&mut self) {
        // process events, update states
    }

    /// Optional continuous run loop.
    pub fn run(&mut self) {
        // loop calling tick()
    }
}
