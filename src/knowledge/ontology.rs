// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\ontology.rs
//
//  Description: Defines concepts, entities, and relationships.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-24
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use std::collections::{HashMap, HashSet};

/// Unique identifier for concepts and entities
pub type Id = usize;

/// Represents a concept (type/class) in the ontology
#[derive(Debug, Clone)]
pub struct Concept {
    pub id: Id,
    pub name: String,
    pub parent_ids: HashSet<Id>, // For inheritance (subclass-of)
    pub attributes: HashMap<String, AttributeType>, // Attributes with types
}

/// Represents an entity (instance) of a concept
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: Id,
    pub concept_id: Id,
    pub attribute_values: HashMap<String, AttributeValue>,
}

/// Supported attribute data types
#[derive(Debug, Clone)]
pub enum AttributeType {
    String,
    Integer,
    Float,
    Boolean,
    Reference(Id), // Reference to another concept/entity
}

/// Values for attributes
#[derive(Debug, Clone)]
pub enum AttributeValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Reference(Id),
}

pub struct Ontology {
    next_id: Id,
    concepts: HashMap<Id, Concept>,
    concepts_by_name: HashMap<String, Id>,
    entities: HashMap<Id, Entity>,
}

impl Ontology {
    pub fn new() -> Self {
        Ontology {
            next_id: 1,
            concepts: HashMap::new(),
            concepts_by_name: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    pub fn add_concept(&mut self, name: &str, parents: &[Id], attributes: HashMap<String, AttributeType>) -> Id {
        let id = self.next_id;
        self.next_id += 1;

        let concept = Concept {
            id,
            name: name.to_string(),
            parent_ids: parents.iter().cloned().collect(),
            attributes,
        };

        self.concepts_by_name.insert(name.to_string(), id);
        self.concepts.insert(id, concept);

        id
    }

    pub fn add_entity(&mut self, concept_id: Id, attribute_values: HashMap<String, AttributeValue>) -> Id {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity {
            id,
            concept_id,
            attribute_values,
        };

        self.entities.insert(id, entity);

        id
    }

    pub fn get_concept(&self, id: Id) -> Option<&Concept> {
        self.concepts.get(&id)
    }

    pub fn get_entity(&self, id: Id) -> Option<&Entity> {
        self.entities.get(&id)
    }
}
