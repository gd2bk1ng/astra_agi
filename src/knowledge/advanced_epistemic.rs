// =============================================================================
//  Astra AGI - Advanced Epistemic Integration Module
//  File: advanced_epistemic.rs
//
//  Description:
//  Integrates Bayesian and Fuzzy Reasoners into the epistemic reasoning pipeline.
//  Provides unified interfaces for belief revision, inference, and uncertainty handling.
//  Includes integration tests for reasoning correctness.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

pub mod bayesian_reasoner;
pub mod fuzzy_reasoner;

use crate::knowledge::extended_ontology::Fact;
use crate::memory::narrative_memory::NarrativeMemory;

pub use bayesian_reasoner::{BayesianNetwork, BBNNode};
pub use fuzzy_reasoner::FuzzyLogic;

/// Unified Epistemic Reasoner combining Bayesian and fuzzy logic.
pub struct AdvancedEpistemicReasoner {
    pub bayesian: BayesianNetwork,
    pub fuzzy: FuzzyLogic,
}

impl AdvancedEpistemicReasoner {
    pub fn new() -> Self {
        AdvancedEpistemicReasoner {
            bayesian: BayesianNetwork::new(),
            fuzzy: FuzzyLogic::new(),
        }
    }

    /// Performs Bayesian marginal probability query.
    pub fn bayesian_marginal(&self, node_id: usize) -> Option<f64> {
        self.bayesian.marginal_probability(node_id)
    }

    /// Performs fuzzy AND operation on two confidences.
    pub fn fuzzy_and(&self, a: f64, b: f64) -> f64 {
        self.fuzzy.fuzzy_and(a, b)
    }

    /// Incorporates new evidence into Bayesian network and logs to narrative memory.
    pub fn incorporate_evidence(&mut self, fact: &Fact, observed_value: bool, narrative: &mut NarrativeMemory) {
        self.bayesian.incorporate_evidence(fact, observed_value, narrative);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::extended_ontology::Provenance;

    #[test]
    fn test_bayesian_and_fuzzy() {
        let mut reasoner = AdvancedEpistemicReasoner::new();

        let node = BBNNode {
            id: 1,
            name: "Rain".to_string(),
            parents: vec![],
            children: vec![],
            cpt: [(vec![], 0.3)].iter().cloned().collect(),
        };
        reasoner.bayesian.add_node(node);

        assert_eq!(reasoner.bayesian_marginal(1), Some(0.3));

        let fuzzy_result = reasoner.fuzzy_and(0.7, 0.9);
        assert!((fuzzy_result - 0.7).abs() < 1e-6);
    }
}
