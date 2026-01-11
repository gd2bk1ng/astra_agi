// ============================================================================
//                       ASTRA AGI • THOUGHT TRACE SYSTEM
//        Transparent Reasoning, Plan Justification & Decision Logging
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Captures structured traces of Astra’s internal reasoning: considered
//       alternatives, chosen strategies, and justification for decisions.
//       This provides explainability, debuggability, and the raw material
//       for reflection and meta-learning.
//
//   Core Functions:
//       • Record thought steps as Astra deliberates
//       • Associate traces with goals, plans, and outcomes
//       • Provide human-readable explanations of decisions
//
//   File:        /src/cognition/thought_trace.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use serde::{Deserialize, Serialize};

/// A single reasoning step in Astra’s thought process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtStep {
    pub message: String,
    pub importance: f32,
}

/// A complete thought trace associated with a goal/plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtTrace {
    pub goal_id: String,
    pub steps: Vec<ThoughtStep>,
}

impl ThoughtTrace {
    pub fn new(goal_id: impl Into<String>) -> Self {
        Self {
            goal_id: goal_id.into(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, message: impl Into<String>, importance: f32) {
        self.steps.push(ThoughtStep {
            message: message.into(),
            importance: importance.clamp(0.0, 1.0),
        });
    }

    pub fn summarize(&self) -> String {
        let mut summary = format!("Thought trace for goal '{}':\n", self.goal_id);
        for step in &self.steps {
            summary.push_str(&format!("- [{}] {}\n", step.importance, step.message));
        }
        summary
    }
}
