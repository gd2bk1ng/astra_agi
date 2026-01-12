// ============================================================================
//                 ASTRA AGI • HEURISTIC REINFORCEMENT ENGINE
//        Adaptive Planning Strategy Selection & Meta-Learning
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Tracks planning success rates and adjusts Astra’s planning heuristics
//       to improve efficiency and adaptability over time.
//
//   Core Functions:
//       • Track GOAP, HTN, and reactive planning success
//       • Adjust biases in PlanningHeuristics
//       • Improve planning efficiency through reinforcement
//
//   File:        /src/cognition/heuristic_reinforcement.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-12
//   Updated:     2026-01-12
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::cognition::{CognitiveState, PlanningHeuristics};

pub fn reinforce_heuristics(state: &mut CognitiveState, success: bool) {
    let delta = if success { 0.01 } else { -0.015 };

    state.heuristics.goap_bias = (state.heuristics.goap_bias + delta).clamp(0.0, 1.0);
    state.heuristics.htn_bias = (state.heuristics.htn_bias + delta * 0.5).clamp(0.0, 1.0);
    state.heuristics.reactive_bias = (state.heuristics.reactive_bias - delta * 0.3).clamp(0.0, 1.0);
}
