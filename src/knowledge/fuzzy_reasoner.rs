// =============================================================================
//  Astra AGI - Fuzzy Logic Reasoner
//  File: fuzzy_reasoner.rs
//
//  Description:
//  Implements fuzzy logic for reasoning with vagueness and partial truths.
//  Complements Bayesian reasoning by allowing graded truth values and linguistic variables.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

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
