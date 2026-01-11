// ============================================================================
//                     ASTRA AGI • FUZZY LOGIC REASONER
//        Graded Truth Evaluation for Vagueness & Partial Information
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Component of Astra’s Knowledge Layer, providing fuzzy logic operations
//       for reasoning under vagueness, ambiguity, and partial truth values.
//       This module complements Bayesian inference by enabling graded truth
//       evaluation, linguistic variables, and smooth transitions between
//       certainty levels—critical for real‑world, imprecise knowledge domains.
//
//   Core Functions:
//       • Compute fuzzy AND, OR, and NOT operations
//       • Support fuzzy implication for rule‑based reasoning
//       • Provide continuous truth‑value handling beyond binary logic
//       • Integrate with probabilistic and epistemic reasoning pipelines
//
//   File:        /src/knowledge/fuzzy_reasoner.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-25
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

pub struct FuzzyLogic;

impl FuzzyLogic {
    pub fn new() -> Self {
        FuzzyLogic {}
    }

    /// Computes fuzzy AND (minimum)
    pub fn fuzzy_and(a: f64, b: f64) -> f64 {
        a.min(b)
    }

    /// Computes fuzzy OR (maximum)
    pub fn fuzzy_or(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    /// Computes fuzzy NOT (1 - value)
    pub fn fuzzy_not(a: f64) -> f64 {
        1.0 - a
    }

    /// Example: fuzzy implication (Mamdani)
    pub fn fuzzy_implication(a: f64, b: f64) -> f64 {
        Self::fuzzy_min(1.0, 1.0 - a + b)
    }

    fn fuzzy_min(a: f64, b: f64) -> f64 {
        a.min(b)
    }
}
