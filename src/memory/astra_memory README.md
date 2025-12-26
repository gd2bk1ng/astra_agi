# Astra Memory Crate

## Overview

Manages narrative memory, episodic storage, and retrieval for long-term learning.

## Main Modules & Functions

- `store_event(event: NarrativeEvent)`  
  Log significant events.

- `recent_events(limit: usize) -> Vec<NarrativeEvent>`  
  Retrieve recent events.

- `query_memory(query: &str) -> Vec<NarrativeEvent>`  
  Search memory.

## Usage

Supports runtime and personality for continuity and reflection.

## Documentation

```bash
cargo doc --open
```

License
MIT OR Apache 2.0
