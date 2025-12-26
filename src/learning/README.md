# Astra Learning Crate

## Overview

This crate implements Astra AGI’s adaptive learning capabilities, including:

- Autodifferentiation for gradient-based optimization.
- Reinforcement learning algorithms.
- Model training and evaluation infrastructure.

## Core Modules

- `autodiff.rs`: Implements autodiff utilities for backpropagation.
- `trainer.rs`: Contains training loops, optimizers, and evaluation metrics.
- `mod.rs`: Exposes the public API and integrates learning components.

## Usage Example

```rust
use astra_learning::{Trainer, Model};

let mut trainer = Trainer::new();
let model = Model::new();

// Train model with data
trainer.train(&model, &training_data).await;
```

Documentation
Generate API docs with:

```rust
cargo doc --open
```
License
MIT OR Apache 2.0
