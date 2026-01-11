// ============================================================================
//                 ASTRA AGI • PLANNING & DECISION SUBSYSTEM (APDS)
//        Root Module for Goal Management, Deliberation & Action Selection
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Serves as the entry point for Astra’s planning and decision‑making
//       subsystem. This module organizes and exposes the core components
//       responsible for goal representation, plan generation, action sequencing,
//       and execution logic. It forms the structural backbone of Astra’s
//       deliberative reasoning and future‑oriented cognition.
//
//   Core Functions:
//       • Define the module layout for planning and decision components
//       • Expose the Planner engine for goal‑driven reasoning
//       • Expose the Executor for action realization and plan enactment
//       • Provide a unified namespace for APDS‑related functionality
//
//   File:        /src/planning/mod.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

pub mod planner;
pub mod executor;
