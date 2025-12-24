// =============================================================================
//  Astra AGI
//  File: examples/persistence_example.rs
//
//  Description: Demonstrates ontology usage with sled persistence.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-26
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use astra_agi::knowledge::{
    Ontology, AttributeType, AttributeValue, RelationshipType,
    storage::SledStorage,
};
use std::collections::HashMap;
use anyhow::Result;

fn main() -> Result<()> {
    let storage = SledStorage::new("example_persistence_db")?;
    let mut ontology = Ontology::new(storage);

    // Add concept and entities
    let mut person_attrs = HashMap::new();
    person_attrs.insert("name".to_string(), AttributeType::String);
    person_attrs.insert("age".to_string(), AttributeType::Integer);
    let person_id = ontology.add_concept("Person", &[], person_attrs);

    let mut alice_attrs = HashMap::new();
    alice_attrs.insert("name".to_string(), AttributeValue::String("Alice".to_string()));
    alice_attrs.insert("age".to_string(), AttributeValue::Integer(30));
    let alice_id = ontology.add_entity(person_id, alice_attrs);

    let mut bob_attrs = HashMap::new();
    bob_attrs.insert("name".to_string(), AttributeValue::String("Bob".to_string()));
    bob_attrs.insert("age".to_string(), AttributeValue::Integer(25));
    let bob_id = ontology.add_entity(person_id, bob_attrs);

    // Add relationship
    ontology.add_relationship(alice_id, bob_id, RelationshipType::FriendOf);

    // Save state
    ontology.save_to_storage()?;

    println!("Ontology saved.");

    // Load ontology from storage
    let storage2 = SledStorage::new("example_persistence_db")?;
    let mut loaded_ontology = Ontology::new(storage2);
    loaded_ontology.load_from_storage()?;

    let alice_loaded = loaded_ontology.get_entity(alice_id).expect("Alice missing");
    println!("Loaded Alice: {:?}", alice_loaded);

    Ok(())
}
