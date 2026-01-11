// ============================================================================
//                ASTRA AGI • BAYESIAN EPISTEMIC REASONER (BBN)
//        Probabilistic Inference & Causal Reasoning Through Belief Networks
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Core component of Astra’s Knowledge Layer, providing a Bayesian Belief
//       Network (BBN) framework for probabilistic reasoning, causal modeling,
//       and uncertainty evaluation. This module enables Astra to update beliefs
//       dynamically as new evidence arrives, supporting mathematically grounded
//       epistemic inference beyond simple confidence scoring.
//
//   Core Functions:
//       • Represent variables and causal dependencies as Bayesian nodes
//       • Evaluate conditional and marginal probabilities
//       • Update beliefs through evidence incorporation
//       • Support probabilistic reasoning across structured knowledge graphs
//       • Provide foundational primitives for higher‑level epistemic modules
//
//   File:        /src/knowledge/bayesian_reasoner.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-25
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use std::collections::HashMap;

/// Represents a node in the Bayesian Network corresponding to a Fact or variable.
#[derive(Debug, Clone)]
pub struct BBNNode {
    pub id: usize,
    pub name: String,
    pub parents: Vec<usize>, // Parent node IDs
    pub cpt: HashMap<Vec<bool>, f64>, // Conditional Probability Table: parent states => P(node=true)
}

impl BBNNode {
    /// Returns the probability of this node being true given parent states.
    pub fn probability_given(&self, parent_states: &[bool]) -> f64 {
        self.cpt.get(parent_states).cloned().unwrap_or(0.0)
    }
}

/// Bayesian Belief Network structure.
pub struct BayesianNetwork {
    pub nodes: HashMap<usize, BBNNode>,
}

impl BayesianNetwork {
    /// Creates a new empty Bayesian Network.
    pub fn new() -> Self {
        BayesianNetwork {
            nodes: HashMap::new(),
        }
    }

    /// Adds a node to the network.
    pub fn add_node(&mut self, node: BBNNode) {
        self.nodes.insert(node.id, node);
    }

    /// Computes the marginal probability of a node being true.
    /// Note: Simplified inference assuming parents are true.
    pub fn marginal_probability(&self, node_id: usize) -> Option<f64> {
        let node = self.nodes.get(&node_id)?;
        if node.parents.is_empty() {
            return node.cpt.get(&vec![]).cloned();
        }
        let parent_states = vec![true; node.parents.len()];
        Some(node.probability_given(&parent_states))
    }

    /// Updates the CPT entry for a node.
    pub fn update_cpt(&mut self, node_id: usize, parent_states: Vec<bool>, prob_true: f64) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.cpt.insert(parent_states, prob_true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbn_node_probability() {
        let mut node = BBNNode {
            id: 1,
            name: "Rain".to_string(),
            parents: vec![],
            cpt: HashMap::new(),
        };
        node.cpt.insert(vec![], 0.2);

        assert_eq!(node.probability_given(&[]), 0.2);
    }

    #[test]
    fn test_bbn_marginal_probability() {
        let mut net = BayesianNetwork::new();
        let node = BBNNode {
            id: 1,
            name: "Rain".to_string(),
            parents: vec![],
            cpt: [(vec![], 0.3)].iter().cloned().collect(),
        };
        net.add_node(node);
        assert_eq!(net.marginal_probability(1), Some(0.3));
    }
}
