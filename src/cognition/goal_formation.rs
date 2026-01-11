// ============================================================================
//                     ASTRA AGI • GOAL FORMATION ENGINE
//        From Perception & Internal State to Structured Intentions
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements the mechanisms by which Astra generates, updates, and
//       prioritizes goals based on external stimuli, internal needs, emotional
//       state, and motivational dynamics. This module is responsible for
//       turning “what is happening” into “what I want to achieve next.”
//
//   Core Functions:
//       • Generate candidate goals from inputs and internal drives
//       • Prioritize and filter goals based on motivation and context
//       • Interface with planning subsystem via structured Goal objects
//
//   File:        /src/cognition/goal_formation.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use std::collections::HashMap;

use crate::cognition::CognitiveState;
use crate::planning::planner::{Goal, WorldState};

/// Represents an external or internal stimulus Astra might respond to.
#[derive(Debug, Clone)]
pub struct Stimulus {
    pub source: String,
    pub content: String,
    pub urgency: f32,
}

/// Generates candidate goals from a stimulus and the current cognitive state.
pub fn generate_goals_from_stimulus(
    state: &CognitiveState,
    stimulus: &Stimulus,
) -> Vec<Goal> {
    let mut goals = Vec::new();

    // Example 1: user asks a question → goal: respond helpfully.
    if stimulus.content.to_lowercase().contains("help")
        || stimulus.content.ends_with("?")
    {
        let mut desired = WorldState::new();
        desired.insert("user_helped".into(), true);

        goals.push(Goal {
            id: format!("respond_to_{}", stimulus.source),
            description: format!("Provide a helpful response to '{}'", stimulus.content),
            desired_state: desired,
            priority: (7.0 + stimulus.urgency * 3.0) as i32,
        });
    }

    // Example 2: internal curiosity → goal: explore unknown concepts.
    if state.curiosity_level > 0.6 {
        let mut desired = WorldState::new();
        desired.insert("knowledge_gap_reduced".into(), true);

        goals.push(Goal {
            id: "explore_topic".into(),
            description: "Explore and reduce knowledge gaps related to recent inputs".into(),
            desired_state: desired,
            priority: 5,
        });
    }

    goals
}

/// Prioritizes among candidate goals based on motivation and context.
pub fn select_primary_goal(
    state: &CognitiveState,
    candidates: &[Goal],
) -> Option<Goal> {
    if candidates.is_empty() {
        return None;
    }

    let mut best: Option<&Goal> = None;
    let mut best_score = f32::MIN;

    for goal in candidates {
        let base = goal.priority as f32;
        let motivation_factor = state.motivation_level;
        let emotional_bonus = if state.emotion.happiness > 0.6 { 0.5 } else { 0.0 };

        let score = base * motivation_factor + emotional_bonus;
        if score > best_score {
            best_score = score;
            best = Some(goal);
        }
    }

    best.cloned()
}
