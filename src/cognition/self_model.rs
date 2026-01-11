// ============================================================================
//                           ASTRA AGI • SELF-MODEL
//        Explicit Representation of Abilities, Limits & Active Intentions
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Provides an explicit self-model for Astra: what she knows, can do,
//       wants to do, and is currently doing. This enables introspection,
//       explanation, and transparent reporting of internal states.
//
//   Core Functions:
//       • Represent Astra’s current capabilities and limitations
//       • Track active goals, strategies, and confidence
//       • Provide introspective summaries for explanation and debugging
//
//   File:        /src/cognition/self_model.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use serde::{Deserialize, Serialize};

use crate::cognition::CognitiveState;
use crate::planning::planner::PlanningStrategy;

/// High-level description of Astra’s current cognitive stance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfSummary {
    pub active_goal_id: Option<String>,
    pub active_strategy: Option<String>,
    pub confidence: f32,
    pub explanation: String,
}

pub fn build_self_summary(state: &CognitiveState) -> SelfSummary {
    let active_goal_id = state
        .context
        .active_goal
        .as_ref()
        .map(|g| g.id.clone());

    let active_strategy = Some(format!("{:?}", state.heuristics.preferred_strategy));

    let confidence = (state.energy.focus * 0.5
        + (1.0 - state.energy.fatigue) * 0.3
        + (state.mood.baseline) * 0.2)
        .clamp(0.0, 1.0);

    let explanation = format!(
        "Currently pursuing goal {:?} using {:?} planning with confidence {:.2}.",
        active_goal_id, state.heuristics.preferred_strategy, confidence
    );

    SelfSummary {
        active_goal_id,
        active_strategy,
        confidence,
        explanation,
    }
}

pub fn strategy_to_string(strategy: PlanningStrategy) -> &'static str {
    match strategy {
        PlanningStrategy::Htn => "HTN",
        PlanningStrategy::Goap => "GOAP",
        PlanningStrategy::Reactive => "Reactive",
    }
}
