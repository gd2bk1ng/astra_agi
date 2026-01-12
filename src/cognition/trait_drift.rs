// ============================================================================
//                       ASTRA AGI • TRAIT DRIFT ENGINE
//        Long-Term Personality Evolution & Behavioral Stabilization
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Adjusts Astra’s personality traits slowly over time based on emotional
//       stability, curiosity, success rates, and reflective insights.
//
//   Core Functions:
//       • Apply small trait adjustments after consolidation cycles
//       • Prevent extreme drift and maintain coherent identity
//       • Integrate emotional stability and curiosity into trait evolution
//
//   File:        /src/cognition/trait_drift.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-12
//   Updated:     2026-01-12
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::cognition::CognitiveState;

pub fn apply_trait_drift(state: &mut CognitiveState) {
    let openness_delta = state.curiosity_level * 0.005;
    let stability_delta = (1.0 - state.emotion.volatility()) * 0.003;

    state.personality.traits.openness =
        (state.personality.traits.openness + openness_delta).clamp(0.0, 1.0);

    state.personality.traits.stability =
        (state.personality.traits.stability + stability_delta).clamp(0.0, 1.0);
}
