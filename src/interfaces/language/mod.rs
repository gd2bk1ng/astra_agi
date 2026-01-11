// ============================================================================
//                     ASTRA AGI • LANGUAGE INTERFACE MODULE
//              Root of Linguistic Processing & Interpretation Layer
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Entry point for Astra’s Language Interface subsystem. This module
//       aggregates and exposes language‑related components such as parsers,
//       tokenizers, semantic processors, and other linguistic utilities that
//       support natural language understanding across the Interfaces Layer.
//
//   Core Functions:
//       • Re-export language processing submodules
//       • Provide unified access to linguistic utilities and pipelines
//       • Support semantic interpretation for downstream cognitive modules
//       • Serve as the integration hub for text‑based language handling
//
//   File:        /src/interfaces/language/mod.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

pub mod astra_lang;
