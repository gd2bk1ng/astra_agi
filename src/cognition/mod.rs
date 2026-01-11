// ============================================================================
//                         ASTRA AGI • COGNITION SUBSYSTEM
//        Global Cognitive State, Control Loop & Meta-Reasoning Orchestration
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Serves as the central coordination layer for Astra’s mind. This module
//       ties together perception, goal formation, motivation, planning,
//       execution, memory, reflection, and meta-learning into a unified
//       cognitive loop. It exposes interfaces for inspecting, evolving, and
//       controlling Astra’s global cognitive state.
//
//   Core Functions:
//       • Define the global CognitiveState representation
//       • Implement the main cognitive loop driving Astra’s behavior
//       • Integrate goal formation, motivation, curiosity, and self-modeling
//       • Provide meta-level processes: reflection, consolidation, mindspace
//
//   File:        /src/cognition/mod.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

pub mod cognitive_state;
pub mod goal_formation;
pub mod motivation;
pub mod curiosity;
pub mod self_model;
pub mod thought_trace;
pub mod cognitive_loop;
pub mod consolidation;
pub mod mindspace;
pub mod episodes;
pub mod learning_adapter;

pub use cognitive_state::*;
pub use goal_formation::*;
pub use motivation::*;
pub use curiosity::*;
pub use self_model::*;
pub use thought_trace::*;
pub use cognitive_loop::*;
pub use consolidation::*;
pub use mindspace::*;
pub use episodes::*;
pub use learning_adapter::*;
