// =============================================================================
//  Astra AGI - Knowledge Module Root
//  File: mod.rs
//
//  Description:
//  Root module for knowledge representation and reasoning.
//  Exposes extended ontology, epistemic reasoner, and advanced epistemic integration.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

mod tests;

pub mod extended_ontology;
pub mod epistemic_reasoner;
pub mod advanced_epistemic;
pub mod bayesian_reasoner;
pub mod fuzzy_reasoner;

pub mod query;
pub mod query_executor;

pub mod ontology;
pub mod reasoner;

pub use ontology::{Ontology, Id, Concept, Entity, AttributeType, AttributeValue};
pub use reasoner::Reasoner;
