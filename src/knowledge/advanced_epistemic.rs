// ============================================================================
//              ASTRA AGI • ADVANCED EPISTEMIC INTEGRATION MODULE
//        Unified Bayesian–Fuzzy Reasoning for Uncertainty & Belief Revision
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Component of Astra’s Knowledge Layer, responsible for integrating
//       probabilistic (Bayesian) and approximate (fuzzy logic) reasoning into
//       a unified epistemic pipeline. This module enables Astra to revise
//       beliefs, handle uncertainty, and perform inference across heterogeneous
//       knowledge sources with both crisp and graded confidence models.
//
//   Core Functions:
//       • Provide Bayesian marginal queries and evidence incorporation
//       • Support fuzzy logic operations for graded truth evaluation
//       • Combine probabilistic and fuzzy reasoning into a unified interface
//       • Log epistemic updates into narrative memory for traceability
//       • Include integration tests ensuring reasoning correctness
//
//   File:        /src/knowledge/advanced_epistemic.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-25
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

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
