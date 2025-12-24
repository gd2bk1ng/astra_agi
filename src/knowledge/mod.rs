// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\mod.rs
//
//  Description: Knowledge module root - re-exports ontology and reasoner.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-24
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

pub mod ontology;
pub mod reasoner;
pub mod query;
pub mod query_executor;

pub use ontology::{Ontology, Id, Concept, Entity, AttributeType, AttributeValue};
pub use reasoner::Reasoner;
