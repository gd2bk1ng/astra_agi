// ============================================================================
//                         ASTRA AGI • LOGIC REASONER CORE
//        Concept Hierarchy Inference & Deductive Reasoning Foundations
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Component of Astra’s Knowledge Layer, providing the foundational logic
//       engine for concept‑based inference and hierarchical reasoning. This
//       module enables Astra to determine class membership, traverse concept
//       taxonomies, and perform structural deductions over the ontology.
//
//   Core Functions:
//       • Determine whether an entity is an instance of a concept or any of its descendants
//       • Recursively evaluate concept hierarchies for subclass relationships
//       • Serve as a lightweight reasoning layer for higher‑order inference modules
//       • Provide extensible hooks for future deductive and rule‑based logic
//
//   File:        /src/knowledge/reasoner.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::knowledge::{Ontology, Id, Concept, Entity};

pub struct Reasoner<'a> {
    pub ontology: &'a Ontology,
}

impl<'a> Reasoner<'a> {
    pub fn new(ontology: &'a Ontology) -> Self {
        Reasoner { ontology }
    }

    /// Example inference: check if entity is instance of a concept or its descendants
    pub fn entity_is_instance_of(&self, entity_id: Id, concept_id: Id) -> bool {
        if let Some(entity) = self.ontology.get_entity(entity_id) {
            self.is_concept_or_subconcept(entity.concept_id, concept_id)
        } else {
            false
        }
    }

    /// Check if concept_a is concept_b or a descendant (subclass) of concept_b
    pub fn is_concept_or_subconcept(&self, concept_a: Id, concept_b: Id) -> bool {
        if concept_a == concept_b {
            return true;
        }
        if let Some(concept) = self.ontology.get_concept(concept_a) {
            for parent_id in &concept.parent_ids {
                if self.is_concept_or_subconcept(*parent_id, concept_b) {
                    return true;
                }
            }
        }
        false
    }

    // Additional reasoning methods can be added here
}

}
