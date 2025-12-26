# Astra Personality Crate

## Overview

Encapsulates Astra’s personality traits, adaptive behavior, and response generation.

## Main Modules & Functions

- `respond_to_input(input: &str) -> String`  
  Generate personality-inflected response.

- `update_traits(feedback: Feedback)`  
  Adjust personality traits based on feedback.

- `get_traits() -> PersonalityTraits`  
  Retrieve current personality state.

## Usage

Called by runtime to shape conversational style.

## Documentation

```bash
cargo doc --open
```

License
MIT OR Apache 2.0
