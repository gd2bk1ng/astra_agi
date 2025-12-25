// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\mod.rs
//
//  Description: Knowledge module root - re-exports ontology and reasoner.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

mod tests;

pub mod query;
pub mod query_executor;

pub mod extended_ontology;
pub mod epistemic_reasoner;

pub mod ontology;
pub mod reasoner;

pub use ontology::{Ontology, Id, Concept, Entity, AttributeType, AttributeValue};
pub use reasoner::Reasoner;
