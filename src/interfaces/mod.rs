// =============================================================================
// Astra AGI - Interfaces Module
// File: mod.rs
//
// Description:
// Central interface module aggregating submodules for API, NLP, and Voice integration.
// Provides unified access points for external communication and interaction layers.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-23
//
// Licensed under MIT OR Apache 2.0
// =============================================================================

pub mod api;
pub mod nlp;
pub mod voice;

pub use api::AstraApi;
pub use nlp::{NlpProcessor, NlpResult};
pub use voice::{VoiceInput, VoiceOutput};
