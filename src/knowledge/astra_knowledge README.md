# Astra Knowledge Crate

## Overview

Manages Astra’s knowledge base, ontologies, and inference mechanisms supporting symbolic and probabilistic reasoning.

## Main Modules & Functions

- `load_ontology(path: &str)`  
  Load domain ontologies for reasoning.

- `infer(query: &str) -> Result<InferenceResult>`  
  Perform inference on knowledge base.

- `update_knowledge(facts: &Vec<Fact>)`  
  Dynamically update knowledge entries.

## Usage

Use to support reasoning and decision-making in runtime and planning.

## Documentation

Generate API docs with:

```bash
cargo doc --open
```

License:
MIT OR Apache 2.0

