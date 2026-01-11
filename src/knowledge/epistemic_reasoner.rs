// ============================================================================
//                 ASTRA AGI • EPISTEMIC REASONER MODULE (ERM)
//        Belief Revision, Uncertainty Management & Knowledge Consistency
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Core component of Astra’s Knowledge Layer, responsible for managing
//       uncertain, conflicting, or evolving knowledge. This module implements
//       belief revision, confidence updating, and uncertainty handling, while
//       integrating tightly with Narrative Memory to record epistemic changes
//       and support reflective reasoning about knowledge evolution.
//
//   Core Functions:
//       • Revise beliefs using confidence thresholds and weighted updates
//       • Reject low‑confidence or unreliable evidence with clear rationale
//       • Incorporate contextual source reliability into belief updates
//       • Combine conflicting facts using consensus‑based aggregation
//       • Log belief updates and rejections into Narrative Memory for traceability
//
//   File:        /src/knowledge/epistemic_reasoner.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-27
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::knowledge::extended_ontology::{Fact, Confidence};
use crate::memory::narrative_memory::NarrativeMemory;
use std::collections::HashMap;

/// Represents the result of a belief revision operation.
pub enum RevisionResult {
    Updated(Fact),
    Rejected(String),
}

/// Epistemic Reasoner struct.
/// Provides methods to revise beliefs, update confidence scores,
/// combine conflicting facts, and log changes to narrative memory.
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

    /// Revised belief with logging to narrative memory.
    ///
    /// Uses `revise_belief` internally for pure logic,
    /// then logs the outcome to the provided NarrativeMemory instance.
pub fn revise_belief_with_logging(&self, current_fact: &Fact, new_fact: &Fact, narrative: &mut NarrativeMemory) -> RevisionResult {
    let result = self.revise_belief(current_fact, new_fact);
    match &result {
        RevisionResult::Updated(fact) => {
            narrative.add_event(
                "belief_updated",
                format!(
                    "Belief revised: {} {} {} with confidence {:.2}",
                    fact.subject, fact.predicate, fact.object, fact.confidence
                ),
                None,
            );
        }
        RevisionResult::Rejected(reason) => {
            narrative.add_event("belief_rejected", reason.clone(), None);
        }
    }
    result
}

    /// Revised belief with contextual source reliability.
    ///
    /// Adjusts new evidence confidence by source reliability before revision.
    pub fn revise_belief_contextual(&self, current_fact: &Fact, new_fact: &Fact, source_reliability: f64) -> RevisionResult {
        let threshold = *self.parameters.get("confidence_threshold").unwrap_or(&0.5);

        let adjusted_confidence = new_fact.confidence as f64 * source_reliability;

        if adjusted_confidence < threshold {
            return RevisionResult::Rejected(format!(
                "Adjusted evidence confidence {:.2} below threshold {:.2}",
                adjusted_confidence, threshold
            ));
        }

        let updated_confidence = ((current_fact.confidence as f64) + adjusted_confidence) / 2.0;

        let updated_fact = Fact {
            confidence: updated_confidence as f32,
            provenance: new_fact.provenance.clone(),
            ..current_fact.clone()
        };

        RevisionResult::Updated(updated_fact)
    }

    /// Combines multiple conflicting facts about the same statement.
    ///
    /// Uses a simple consensus approach weighted by confidence and recency.
    ///
    /// # Arguments
    /// * `facts` - Slice of facts to combine.
    ///
    /// # Returns
    /// * A new fact representing the combined belief or None if facts is empty.
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
