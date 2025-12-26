# Astra AGI Interfaces Module

## Overview

This module provides the external-facing interfaces for Astra AGI, including:

- **API Interface:** REST endpoints for chat and interaction.
- **NLP Interface:** Natural language processing for intent and entity extraction.
- **Voice Interface:** Speech recognition and text-to-speech capabilities.

## API Interface

- Endpoint: `/chat` (POST)  
  Accepts chat messages and returns Astra’s responses, emotional state, personality traits, and recent narrative events.

## NLP Interface

- Endpoint: `/nlp` (POST)  
  Processes raw text to extract intents, entities, and confidence scores.

## Voice Interface

- Endpoint: `/voice/listen` (GET)  
  Captures voice input and returns transcribed text.

- Endpoint: `/voice/speak` (POST)  
  Accepts text to be spoken aloud via TTS.

## How to Use

1. Instantiate the interfaces with a shared `Runtime` instance.
2. Mount routes in Actix-web server as shown in the example.
3. Use JSON requests/responses to communicate with Astra AGI.

## Wow Factor Features to Add

- **Humor Module:** Integrate a joke generator or humor classifier to make Astra funny and engaging.
- **Contextual Wit:** Use NLP to detect conversational context and insert witty remarks.
- **Adaptive Personality:** Vary humor style based on user preferences or mood.
- **Multi-modal Interaction:** Combine voice, text, and visual cues for richer engagement.
- **Emotion-aware Responses:** Tailor jokes or comments to Astra’s current emotional state for authenticity.

## Next Steps

- Extend NLP with state-of-the-art models (e.g., transformers).
- Integrate real speech recognition and TTS backends.
- Add personalization and learning from user feedback.

## License

MIT OR Apache 2.0
