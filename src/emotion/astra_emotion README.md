# Astra Emotion Crate

## Overview

Models Astra’s emotional state and its influence on reasoning and personality.

## Main Modules & Functions

- `update_emotion(event: EmotionEvent)`  
  Update emotional state.

- `get_current_emotion() -> EmotionState`  
  Get current emotion profile.

- `influence_decision(decision: &mut Decision)`  
  Modify decisions based on emotions.

## Usage

Integrates with runtime and personality for affective computing.

## Documentation

```bash
cargo doc --open
```

License
MIT OR Apache 2.0
