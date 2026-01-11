// ============================================================================
//                         ASTRA AGI • PERSONALITY SUBSYSTEM
//        Root Module for Behavioral Traits & Affective Interaction Engine
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Serves as the entry point for Astra’s Personality subsystem. This
//       module organizes and exposes the components responsible for adaptive
//       behavior, affective modulation, humor generation, and expressive
//       interaction patterns. It defines the structural foundation for Astra’s
//       persona and conversational style.
//
//   Core Functions:
//       • Define the module layout for personality‑related components
//       • Expose personality traits, affective logic, and humor systems
//       • Provide a unified namespace for expressive behavior modules
//       • Establish the basis for future emotional and stylistic engines
//
//   File:        /src/personality/mod.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-25
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

pub mod personality;
pub mod humor;

pub use personality::*;
pub use humor::*;
