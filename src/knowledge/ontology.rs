// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\ontology.rs
//
//  Description: Defines concepts, entities, relationships, and adds indexing and graph adjacency.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

pub type Id = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: Id,
    pub name: String,
    pub parent_ids: HashSet<Id>,
    pub attributes: HashMap<String, AttributeType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Id,
    pub concept_id: Id,
    pub attribute_values: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributeType {
    String,
    Integer,
    Float,
    Boolean,
    Reference(Id),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributeValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Reference(Id),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RelationshipType {
    ParentOf,
    ChildOf,
    FriendOf,
    WorksAt,
    RelatedTo,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Id,
    pub from_entity: Id,
    pub to_entity: Id,
    pub rel_type: RelationshipType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ontology {
    next_id: Id,

    concepts: HashMap<Id, Concept>,
    concepts_by_name: HashMap<String, Id>,

    entities: HashMap<Id, Entity>,

    relationships: HashMap<Id, Relationship>,

    // --- NEW INDEXES ---

    // Map attribute name -> attribute value -> set of entity IDs
    attribute_index: HashMap<String, HashMap<AttributeValue, HashSet<Id>>>,

    // Map from_entity -> rel_type -> set of relationship IDs
    relationship_index: HashMap<Id, HashMap<RelationshipType, HashSet<Id>>>,

    // Adjacency list: entity -> neighbors (to_entity)
    adjacency_list: HashMap<Id, HashSet<Id>>,
}

impl Ontology {
    pub fn new() -> Self {
        Ontology {
            next_id: 1,
            concepts: HashMap::new(),
            concepts_by_name: HashMap::new(),
            entities: HashMap::new(),
            relationships: HashMap::new(),

            attribute_index: HashMap::new(),
            relationship_index: HashMap::new(),
            adjacency_list: HashMap::new(),
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
            attribute_values: attribute_values.clone(),
        };

        self.entities.insert(id, entity);

        // Update attribute index
        for (attr_name, attr_value) in attribute_values.into_iter() {
            self.attribute_index
                .entry(attr_name)
                .or_default()
                .entry(attr_value)
                .or_default()
                .insert(id);
        }

        id
    }

    pub fn add_relationship(&mut self, from_entity: Id, to_entity: Id, rel_type: RelationshipType) -> Id {
        let id = self.next_id;
        self.next_id += 1;

        let relationship = Relationship {
            id,
            from_entity,
            to_entity,
            rel_type: rel_type.clone(),
        };

        self.relationships.insert(id, relationship);

        // Update relationship index
        self.relationship_index
            .entry(from_entity)
            .or_default()
            .entry(rel_type)
            .or_default()
            .insert(id);

        // Update adjacency list
        self.adjacency_list.entry(from_entity).or_default().insert(to_entity);

        id
    }

    // Efficient lookup for entities by attribute value using index
    pub fn find_entities_by_attribute_indexed(&self, attr_name: &str, attr_value: &AttributeValue) -> Vec<&Entity> {
        if let Some(val_map) = self.attribute_index.get(attr_name) {
            if let Some(entity_ids) = val_map.get(attr_value) {
                entity_ids.iter().filter_map(|id| self.entities.get(id)).collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    // Efficient retrieval of relationships from an entity by type
    pub fn get_relationships_indexed(&self, entity_id: Id, rel_type_filter: Option<RelationshipType>) -> Vec<&Relationship> {
        if let Some(rel_map) = self.relationship_index.get(&entity_id) {
            match rel_type_filter {
                Some(rel_type) => {
                    if let Some(rel_ids) = rel_map.get(&rel_type) {
                        rel_ids.iter().filter_map(|id| self.relationships.get(id)).collect()
                    } else {
                        vec![]
                    }
                }
                None => rel_map.values().flat_map(|rel_id_set| {
                    rel_id_set.iter().filter_map(|id| self.relationships.get(id))
                }).collect(),
            }
        } else {
            vec![]
        }
    }

    // Example: Get neighbors of an entity (adjacent entities)
    pub fn get_neighbors(&self, entity_id: Id) -> Vec<&Entity> {
        if let Some(neighbors) = self.adjacency_list.get(&entity_id) {
            neighbors.iter().filter_map(|id| self.entities.get(id)).collect()
        } else {
            vec![]
        }
    }

    // Existing methods like get_concept, get_entity, find_entities_by_concept etc. remain unchanged
}
