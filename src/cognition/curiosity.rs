// ============================================================================
//                        ASTRA AGI • CURIOSITY ENGINE
//        Knowledge Gap Detection, Intrinsic Motivation & Exploration Drive
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Models Astra’s curiosity as an intrinsic drive to reduce uncertainty,
//       explore novel states, and close knowledge gaps. This engine influences
//       goal formation, prioritization, and learning behaviors.
//
//   Core Functions:
//       • Estimate knowledge gaps from recent interactions
//       • Adjust curiosity level based on novelty and surprise
//       • Generate curiosity-driven exploration signals for goal formation
//
//   File:        /src/cognition/curiosity.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::cognition::CognitiveState;

/// Simple estimate of curiosity based on perceived novelty and uncertainty.
/// In a full system, this would integrate prediction errors, model confidence,
/// and unexplained variance from the Learning subsystem.
pub fn update_curiosity(state: &mut CognitiveState, novelty_score: f32) {
    let novelty = novelty_score.clamp(0.0, 1.0);
    let base = state.curiosity_level;

    let updated = (base * 0.8) + (novelty * 0.2);
    state.curiosity_level = updated.clamp(0.0, 1.0);
}
