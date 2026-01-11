// ============================================================================
//                         ASTRA AGI • INTERFACES MODULE
//                 Unified Gateway for External Communication Layers
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Root of Astra’s Interfaces Layer. This module aggregates and exposes
//       the system’s external interaction capabilities—including API, NLP,
//       and Voice integration—providing a cohesive entry point for all
//       communication channels that interface with the cognitive runtime.
//
//   Core Functions:
//       • Re-export API, NLP, and Voice submodules
//       • Provide unified access to external interaction mechanisms
//       • Coordinate message flow into the cognitive pipeline
//       • Serve as the integration hub for all user-facing communication
//
//   File:        /src/interfaces/mod.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

pub mod api;
pub mod nlp;
pub mod voice;

pub use api::AstraApi;
pub use nlp::{NlpProcessor, NlpResult};
pub use voice::{VoiceInput, VoiceOutput};
