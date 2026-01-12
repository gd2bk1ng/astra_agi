// ============================================================================
//                     ASTRA AGI • SLEEP CYCLE SCHEDULER
//        Cognitive Load Monitoring, Rest Cycles & Consolidation Triggers
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Monitors cognitive load, fatigue, and emotional volatility to trigger
//       “sleep cycles” where Astra performs consolidation, reflection, and
//       trait drift updates.
//
//   Core Functions:
//       • Detect when Astra needs rest
//       • Trigger consolidation + reflection + drift
//       • Reset fatigue and stabilize mood
//
//   File:        /src/cognition/sleep_scheduler.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-12
//   Updated:     2026-01-12
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::cognition::{
    CognitiveState, run_consolidation_cycle, apply_trait_drift, apply_mood_curve,
};

pub fn should_sleep(state: &CognitiveState) -> bool {
    state.energy.fatigue > 0.7 || state.emotion.volatility() > 0.6
}

pub fn run_sleep_cycle(state: &mut CognitiveState, time_hours: f32) {
    run_consolidation_cycle(state).ok();
    apply_trait_drift(state);
    apply_mood_curve(state, time_hours);

    state.energy.fatigue = (state.energy.fatigue * 0.5).clamp(0.0, 1.0);
}
