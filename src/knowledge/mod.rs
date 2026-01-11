// ============================================================================
//                        ASTRA AGI • KNOWLEDGE MODULE ROOT
//        Unified Entry Point for Ontology, Reasoning & Epistemic Systems
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Root of Astra’s Knowledge Layer. This module organizes and exposes all
//       core components related to structured knowledge representation,
//       epistemic reasoning, ontology management, and advanced inference
//       mechanisms. It serves as the central integration hub through which
//       higher‑level cognitive systems access Astra’s semantic and epistemic
//       capabilities.
//
//   Core Functions:
//       • Re-export ontology, reasoning, and epistemic modules
//       • Provide unified access to structured knowledge and inference engines
//       • Integrate Bayesian, fuzzy, and hybrid epistemic reasoning pipelines
//       • Serve as the foundation for querying, updating, and interpreting
//         Astra’s evolving knowledge base
//
//   File:        /src/knowledge/mod.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

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
