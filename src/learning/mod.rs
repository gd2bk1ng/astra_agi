// ============================================================================
//                         ASTRA AGI • LEARNING SUBSYSTEM
//        Adaptive Learning Algorithms & Training Infrastructure Overview
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Serves as the root module for Astra’s Learning crate. This file
//       organizes and exposes the core components responsible for adaptive
//       learning, gradient‑based optimization, reinforcement learning,
//       and model‑training utilities. It provides the structural entry point
//       for all learning‑related modules within the subsystem.
//
//   Core Functions:
//       • Define the module layout for the Learning crate
//       • Expose autodiff, training, and reinforcement learning components
//       • Provide a unified namespace for Astra’s adaptive learning logic
//       • Establish the foundation for future model‑training pipelines
//
//   File:        /src/learning/mod.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

pub mod autodiff;
pub mod trainer;

pub use autodiff::*;
pub use trainer::*;
