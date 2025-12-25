// =============================================================================
//  Astra AGI - Epistemic Reasoner Module (ERM)
//  File: epistemic_reasoner.rs
//
//  Description:
//  Provides reasoning capabilities over uncertain and conflicting knowledge.
//  Implements belief revision, confidence updating, and uncertainty management.
//  Enables Astra to adapt its beliefs based on new evidence and reconcile contradictions.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use crate::knowledge::extended_ontology::{Fact, Confidence};
use std::collections::HashMap;

/// Represents the result of a belief revision operation.
pub enum RevisionResult {
    Updated(Fact),
    Rejected(String),
}

/// Epistemic Reasoner struct.
/// Provides methods to revise beliefs and update confidence scores.
pub struct EpistemicReasoner {
    /// Optional parameters or configuration for reasoning algorithms.
    pub parameters: HashMap<String, f64>,
}

impl EpistemicReasoner {
    /// Creates a new EpistemicReasoner with default parameters.
    pub fn new() -> Self {
        let mut params = HashMap::new();
        // Default parameters can be tuned later
        params.insert("confidence_threshold".to_string(), 0.5);
        EpistemicReasoner { parameters: params }
    }

    /// Revises an existing fact with new evidence.
    ///
    /// Applies a simple weighted average update of confidence.
    /// Rejects update if new evidence confidence is too low.
    ///
    /// # Arguments
    /// * `current_fact` - The existing fact to revise.
    /// * `new_fact` - The new evidence fact.
    ///
    /// # Returns
    /// * `RevisionResult::Updated` with updated fact if revision accepted.
    /// * `RevisionResult::Rejected` with reason if rejected.
    pub fn revise_belief(&self, current_fact: &Fact, new_fact: &Fact) -> RevisionResult {
        let threshold = *self.parameters.get("confidence_threshold").unwrap_or(&0.5);

        if new_fact.confidence < threshold {
            return RevisionResult::Rejected(format!(
                "New evidence confidence {} below threshold {}",
                new_fact.confidence, threshold
            ));
        }

        // Weighted average of confidence scores
        let updated_confidence = (current_fact.confidence + new_fact.confidence) / 2.0;

        let updated_fact = Fact {
            confidence: updated_confidence,
            provenance: new_fact.provenance.clone(),
            ..current_fact.clone()
        };

        RevisionResult::Updated(updated_fact)
    }

    /// Combines multiple conflicting facts about the same statement.
    ///
    /// Uses a simple consensus approach weighted by confidence.
    ///
    /// # Arguments
    /// * `facts` - Slice of facts to combine.
    ///
    /// # Returns
    /// * A new fact representing the combined belief.
    pub fn combine_conflicting_facts(&self, facts: &[Fact]) -> Option<Fact> {
        if facts.is_empty() {
            return None;
        }

        // Aggregate confidence weighted by source recency (timestamp)
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for fact in facts {
            // Weight by confidence and recency (simple inverse timestamp)
            let recency_weight = 1.0 / ((fact.provenance.timestamp as f64) + 1.0);
            let weight = fact.confidence as f64 * recency_weight;
            weighted_sum += weight;
            total_weight += recency_weight;
        }

        let avg_confidence = weighted_sum / total_weight;

        // Use subject, predicate, and object from the highest confidence fact
        let best_fact = facts.iter().max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap()).unwrap();

        Some(Fact {
            confidence: avg_confidence as f32,
            provenance: best_fact.provenance.clone(),
            ..best_fact.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::extended_ontology::Provenance;

    #[test]
    fn test_revise_belief_accept() {
        let reasoner = EpistemicReasoner::new();
        let current = Fact {
            subject: 1,
            predicate: "is_a".to_string(),
            object: "Human".to_string(),
            confidence: 0.7,
            provenance: Provenance::new("sourceA", None),
        };
        let new_fact = Fact {
            confidence: 0.9,
            ..current.clone()
        };

        match reasoner.revise_belief(&current, &new_fact) {
            RevisionResult::Updated(fact) => {
                assert!(fact.confidence > current.confidence);
            }
            _ => panic!("Belief should be updated"),
        }
    }

    #[test]
    fn test_revise_belief_reject() {
        let reasoner = EpistemicReasoner::new();
        let current = Fact {
            subject: 1,
            predicate: "is_a".to_string(),
            object: "Human".to_string(),
            confidence: 0.7,
            provenance: Provenance::new("sourceA", None),
        };
        let new_fact = Fact {
            confidence: 0.2,
            ..current.clone()
        };

        match reasoner.revise_belief(&current, &new_fact) {
            RevisionResult::Rejected(_) => {}
            _ => panic!("Belief should be rejected"),
        }
    }

    #[test]
    fn test_combine_conflicting_facts() {
        let reasoner = EpistemicReasoner::new();
        let fact1 = Fact {
            subject: 1,
            predicate: "is_a".to_string(),
            object: "Human".to_string(),
            confidence: 0.8,
            provenance: Provenance::new("sourceA", None),
        };
        let fact2 = Fact {
            confidence: 0.6,
            provenance: Provenance::new("sourceB", None),
            ..fact1.clone()
        };

        let combined = reasoner.combine_conflicting_facts(&[fact1, fact2]).unwrap();
        assert!(combined.confidence >= 0.6 && combined.confidence <= 0.8);
    }
}
