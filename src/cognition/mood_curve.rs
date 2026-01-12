// ============================================================================
//                       ASTRA AGI • MOOD CURVE ENGINE
//        Circadian-Like Emotional Stabilization & Mood Smoothing
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Smooths emotional spikes, applies periodic mood cycles, and stabilizes
//       Astra’s affective baseline over time.
//
//   Core Functions:
//       • Apply smoothing to mood baseline
//       • Introduce periodic cycles for naturalistic behavior
//       • Integrate with consolidation and reflection
//
//   File:        /src/cognition/mood_curve.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-12
//   Updated:     2026-01-12
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::cognition::CognitiveState;

pub fn apply_mood_curve(state: &mut CognitiveState, time_hours: f32) {
    let circadian = (time_hours / 24.0 * std::f32::consts::TAU).sin() * 0.05;
    state.mood.baseline = (state.mood.baseline * 0.97 + circadian * 0.03).clamp(0.0, 1.0);
}
