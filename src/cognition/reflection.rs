// ============================================================================
//                         ASTRA AGI • REFLECTION ENGINE
//        Meta-Reasoning, Self-Evaluation & Cognitive Improvement Signals
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements Astra’s reflective reasoning layer. This module analyzes
//       thought traces, emotional patterns, planning efficiency, and recent
//       outcomes to generate “reflection deltas” that improve heuristics,
//       personality traits, and emotional stability.
//
//   Core Functions:
//       • Analyze thought traces for inefficiencies and blind spots
//       • Detect emotional instability or motivational imbalance
//       • Produce heuristic and trait adjustments for consolidation
//
//   File:        /src/cognition/reflection.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-12
//   Updated:     2026-01-12
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::cognition::{CognitiveState, ThoughtTrace};

#[derive(Debug, Clone)]
pub struct ReflectionDelta {
    pub planning_bias_adjustment: f32,
    pub emotional_stability_adjustment: f32,
    pub curiosity_adjustment: f32,
}

pub fn reflect_on_episode(state: &CognitiveState, trace: &ThoughtTrace, success: bool) -> ReflectionDelta {
    let avg_importance = trace.steps.iter().map(|s| s.importance).sum::<f32>()
        / (trace.steps.len().max(1) as f32);

    let planning_bias_adjustment = if success { 0.02 } else { -0.03 };
    let emotional_stability_adjustment = if avg_importance > 0.7 { 0.01 } else { -0.01 };
    let curiosity_adjustment = if !success { 0.02 } else { 0.0 };

    ReflectionDelta {
        planning_bias_adjustment,
        emotional_stability_adjustment,
        curiosity_adjustment,
    }
}
