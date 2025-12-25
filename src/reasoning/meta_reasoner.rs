// =============================================================================
//  Astra AGI - Meta-Reasoning Layer (MRL)
//  File: meta_reasoner.rs
//
//  Description:
//  Supervisory meta-reasoning system that monitors and adapts Astra's reasoning paradigms.
//  Supports dynamic selection and blending of reasoning methodologies such as Positivism,
//  Constructivism, and Pragmatism based on task, context, and feedback.
//
//  Enables Astra’s philosophical self-awareness and adaptive cognitive control.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-24
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use std::collections::HashMap;

/// Enum representing supported reasoning paradigms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReasoningParadigm {
    Positivism,     // Empirical, data-driven, objective reasoning
    Constructivism, // Knowledge constructed via interaction and experience
    Pragmatism,     // Practical, outcome-focused reasoning
}

/// Represents the current state of meta-reasoning control.
#[derive(Debug)]
pub struct MetaReasoner {
    /// Current weights or preferences for each reasoning paradigm.
    paradigm_weights: HashMap<ReasoningParadigm, f64>,

    /// History of reasoning paradigm usage and task outcomes.
    usage_history: Vec<(ReasoningParadigm, bool)>, // (Paradigm used, success)
}

impl MetaReasoner {
    /// Creates a new MetaReasoner with default equal weights.
    pub fn new() -> Self {
        let mut paradigm_weights = HashMap::new();
        paradigm_weights.insert(ReasoningParadigm::Positivism, 1.0);
        paradigm_weights.insert(ReasoningParadigm::Constructivism, 1.0);
        paradigm_weights.insert(ReasoningParadigm::Pragmatism, 1.0);

        MetaReasoner {
            paradigm_weights,
            usage_history: Vec::new(),
        }
    }

    /// Selects the reasoning paradigm(s) to apply for a given task/context.
    ///
    /// Returns a weighted list of paradigms to blend or prioritize.
    pub fn select_paradigms(&self) -> Vec<(ReasoningParadigm, f64)> {
        // Normalize weights to sum to 1.0
        let total_weight: f64 = self.paradigm_weights.values().sum();
        self.paradigm_weights.iter()
            .map(|(&p, &w)| (p, w / total_weight))
            .collect()
    }

    /// Updates paradigm weights based on task outcome feedback.
    ///
    /// # Arguments
    /// * `paradigm` - The paradigm used.
    /// * `success` - Whether the reasoning was successful (true) or not (false).
    pub fn update_weights(&mut self, paradigm: ReasoningParadigm, success: bool) {
        // Simple reinforcement learning style update
        let current_weight = self.paradigm_weights.entry(paradigm).or_insert(1.0);
        if success {
            *current_weight *= 1.1; // Increase weight by 10%
        } else {
            *current_weight *= 0.9; // Decrease weight by 10%
        }
        self.usage_history.push((paradigm, success));
    }

pub fn update_weights_with_logging(&mut self, paradigm: ReasoningParadigm, success: bool, narrative: &mut NarrativeMemory) {
    self.update_weights(paradigm, success);
    narrative.add_event(
        "paradigm_update",
        format!("Paradigm {:?} updated with success={}", paradigm, success),
        None,
    );
}

/// Returns a human-readable summary of current paradigm weights.
    pub fn summary(&self) -> String {
        let mut s = String::from("MetaReasoner Paradigm Weights:\n");
        for (paradigm, weight) in &self.paradigm_weights {
            s.push_str(&format!("  {:?}: {:.3}\n", paradigm, weight));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paradigm_selection_normalization() {
        let mr = MetaReasoner::new();
        let selected = mr.select_paradigms();
        let sum: f64 = selected.iter().map(|(_, w)| w).sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_weight_update_increases_and_decreases() {
        let mut mr = MetaReasoner::new();
        let old_weight = mr.paradigm_weights[&ReasoningParadigm::Pragmatism];
        mr.update_weights(ReasoningParadigm::Pragmatism, true);
        assert!(mr.paradigm_weights[&ReasoningParadigm::Pragmatism] > old_weight);
        mr.update_weights(ReasoningParadigm::Pragmatism, false);
        assert!(mr.paradigm_weights[&ReasoningParadigm::Pragmatism] < old_weight * 1.1);
    }
}
