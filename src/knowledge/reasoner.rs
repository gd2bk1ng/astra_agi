// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\reasoner.rs
//
//  Description: Logic engine for inference and deduction.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-24
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

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
