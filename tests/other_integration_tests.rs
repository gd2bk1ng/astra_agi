// =============================================================================
//  Astra AGI
//  File: tests/other_integration_tests.rs
//
//  Description: Integration tests for Ontology persistence and relationship querying.
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

#[test]
fn test_persistence_and_relationships() -> Result<()> {
    let storage = SledStorage::new("test_integration_db")?;
    let mut ontology = Ontology::new(storage);

    // Add concept "Device"
    let mut device_attrs = HashMap::new();
    device_attrs.insert("model".to_string(), AttributeType::String);
    let device_id = ontology.add_concept("Device", &[], device_attrs);

    // Add concept "Person"
    let mut person_attrs = HashMap::new();
    person_attrs.insert("name".to_string(), AttributeType::String);
    let person_id = ontology.add_concept("Person", &[], person_attrs);

    // Add entities
    let mut phone_attrs = HashMap::new();
    phone_attrs.insert("model".to_string(), AttributeValue::String("Pixel 7".to_string()));
    let phone_id = ontology.add_entity(device_id, phone_attrs);

    let mut alice_attrs = HashMap::new();
    alice_attrs.insert("name".to_string(), AttributeValue::String("Alice".to_string()));
    let alice_id = ontology.add_entity(person_id, alice_attrs);

    // Add relationship: Alice owns Pixel 7
    ontology.add_relationship(alice_id, phone_id, RelationshipType::Custom("owns".to_string()));

    // Save and reload
    ontology.save_to_storage()?;

    let storage2 = SledStorage::new("test_integration_db")?;
    let mut loaded_ontology = Ontology::new(storage2);
    loaded_ontology.load_from_storage()?;

    // Verify entity and relationship
    let alice_loaded = loaded_ontology.get_entity(alice_id).expect("Alice entity missing");
    assert_eq!(alice_loaded.attribute_values.get("name").unwrap(), &AttributeValue::String("Alice".to_string()));

    let relationships = loaded_ontology.get_relationships_indexed(alice_id, None);
    assert!(relationships.iter().any(|rel| rel.rel_type == RelationshipType::Custom("owns".to_string())));

    Ok(())
}
