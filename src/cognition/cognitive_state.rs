// ============================================================================
//                      ASTRA AGI • GLOBAL COGNITIVE STATE
//        Unified Representation of Mind, Context & Control Parameters
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Defines Astra’s holistic cognitive state, capturing personality,
//       emotion, goals, plans, memory links, heuristics, curiosity, and
//       meta-learning parameters in a single structure. This state forms the
//       backbone of Astra’s mind and is the primary data model for the
//       cognitive loop.
//
//   Core Functions:
//       • Represent current goals, active plans, and execution context
//       • Maintain personality, emotion, and mood influences
//       • Track planning heuristics and reflection-derived meta-parameters
//       • Provide a serializable snapshot of Astra’s internal mind state
//
//   File:        /src/cognition/cognitive_state.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::planning::planner::{Goal, Plan, PlanningStrategy};
use crate::personality::personality::{Personality, PersonalityTraits};
use crate::personality::emotion::{EmotionDynamics, EmotionState, Mood};

/// High-level cognitive heuristics influenced by reflection and meta-learning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningHeuristics {
    pub preferred_strategy: PlanningStrategy,
    pub goap_bias: f32,
    pub htn_bias: f32,
    pub reactive_bias: f32,
}

impl Default for PlanningHeuristics {
    fn default() -> Self {
        Self {
            preferred_strategy: PlanningStrategy::Goap,
            goap_bias: 0.6,
            htn_bias: 0.3,
            reactive_bias: 0.1,
        }
    }
}

/// Represents the current motivational context and cognitive energy level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveEnergy {
    pub focus: f32,     // 0..1
    pub fatigue: f32,   // 0..1
    pub load: f32,      // 0..1 (cognitive load)
}

impl CognitiveEnergy {
    pub fn baseline() -> Self {
        Self {
            focus: 0.7,
            fatigue: 0.2,
            load: 0.3,
        }
    }
}

/// Represents Astra’s active cognitive context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveContext {
    pub active_goal: Option<Goal>,
    pub active_plan: Option<Plan>,

    // Instant cannot be serialized; skip it.
    #[serde(skip)]
    pub last_update: Instant,
}

/// Global cognitive state snapshot for Astra.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub personality: Personality,
    pub personality_traits: PersonalityTraits,
    pub emotion: EmotionState,
    pub mood: Mood,
    pub energy: CognitiveEnergy,
    pub heuristics: PlanningHeuristics,
    pub context: CognitiveContext,
    pub curiosity_level: f32,
    pub motivation_level: f32,
}

impl CognitiveState {
    /// Creates a new default cognitive state.
    pub fn new() -> Self {
        let personality = Personality::new();
        let traits = personality.traits.clone();
        let dynamics = EmotionDynamics::new();

        Self {
            personality,
            personality_traits: traits,
            emotion: dynamics.current.clone(),
            mood: dynamics.mood.clone(),
            energy: CognitiveEnergy::baseline(),
            heuristics: PlanningHeuristics::default(),
            context: CognitiveContext {
                active_goal: None,
                active_plan: None,
                last_update: Instant::now(),
            },
            curiosity_level: 0.5,
            motivation_level: 0.7,
        }
    }

    /// Updates internal timestamps and can be used to drive decay models.
    pub fn touch(&mut self) {
        self.context.last_update = Instant::now();
    }
}
