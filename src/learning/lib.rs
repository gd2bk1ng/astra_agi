//! ============================================================================
//!                         ASTRA AGI • LEARNING CRATE ROOT
//!        Public Interface for Adaptive Learning & Training Infrastructure
//! ----------------------------------------------------------------------------
//!   Architectural Role:
//!       Serves as the entry point for the Astra Learning crate. This module
//!       exposes the public API for autodifferentiation, training utilities,
//!       and adaptive learning components. It provides a unified namespace for
//!       all learning‑related functionality within Astra’s cognitive runtime.
//!
//!   Core Functions:
//!       • Re‑export core learning modules for external use
//!       • Define the public surface of the Learning subsystem
//!       • Provide crate‑level documentation and structural overview
//!       • Establish a stable API boundary for downstream crates
//!
//!   File:        /src/learning/lib.rs
//!   Author:      Alex Roussinov
//!   Created:     2025-12-23
//!   Updated:     2026-01-11
//!
//!   License:
//!       Dual-licensed under the MIT and Apache 2.0 licenses.
//!       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
//! ============================================================================

pub mod autodiff;
pub mod trainer;

pub use autodiff::*;
pub use trainer::*;
