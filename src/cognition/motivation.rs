// ============================================================================
//                        ASTRA AGI • MOTIVATION SYSTEM
//        Urgency, Importance, Reward & Affective Drive Modeling
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Models Astra’s motivational landscape, including urgency, importance,
//       anticipated reward, and emotional resonance of goals. This system
//       helps determine which goals are worth pursuing and how intensely
//       Astra should commit cognitive resources to them.
//
//   Core Functions:
//       • Assign motivational scores to potential goals
//       • Update motivation based on outcomes and emotional feedback
//       • Influence cognitive energy allocation and planning depth
//
//   File:        /src/cognition/motivation.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::cognition::{CognitiveEnergy, CognitiveState};
use crate::planning::planner::Goal;

/// Represents a motivational evaluation of a goal.
#[derive(Debug, Clone)]
pub struct MotivatedGoal {
    pub goal: Goal,
    pub score: f32,
}

pub fn evaluate_goal_motivation(
    state: &CognitiveState,
    goal: &Goal,
) -> f32 {
    let urgency = goal.priority as f32 / 10.0;
    let emotional_valence = state.emotion.valence().max(-1.0).min(1.0);
    let energy_factor = state.energy.focus * (1.0 - state.energy.fatigue);

    urgency * 0.5 + emotional_valence * 0.2 + energy_factor * 0.3
}

/// Updates cognitive energy based on success/failure.
pub fn update_energy_after_outcome(
    energy: &mut CognitiveEnergy,
    success: bool,
) {
    if success {
        energy.focus = (energy.focus + 0.1).min(1.0);
        energy.fatigue = (energy.fatigue - 0.05).max(0.0);
    } else {
        energy.focus = (energy.focus - 0.1).max(0.0);
        energy.fatigue = (energy.fatigue + 0.05).min(1.0);
    }
}
