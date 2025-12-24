// =============================================================================
//  Astra AGI
//  File: examples/graph_traversal_example.rs
//
//  Description: Demonstrates graph traversal and shortest path in Ontology.
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
    let storage = SledStorage::new("example_graph_db")?;
    let mut ontology = Ontology::new(storage);

    // Add concept "Person"
    let mut person_attrs = HashMap::new();
    person_attrs.insert("name".to_string(), AttributeType::String);
    let person_id = ontology.add_concept("Person", &[], person_attrs);

    // Add entities
    let mut alice_attrs = HashMap::new();
    alice_attrs.insert("name".to_string(), AttributeValue::String("Alice".to_string()));
    let alice_id = ontology.add_entity(person_id, alice_attrs);

    let mut bob_attrs = HashMap::new();
    bob_attrs.insert("name".to_string(), AttributeValue::String("Bob".to_string()));
    let bob_id = ontology.add_entity(person_id, bob_attrs);

    let mut carol_attrs = HashMap::new();
    carol_attrs.insert("name".to_string(), AttributeValue::String("Carol".to_string()));
    let carol_id = ontology.add_entity(person_id, carol_attrs);

    // Add relationships (edges)
    ontology.add_relationship(alice_id, bob_id, RelationshipType::FriendOf);
    ontology.add_relationship(bob_id, carol_id, RelationshipType::FriendOf);

    // Test BFS traversal starting from Alice
    let bfs_result = ontology.bfs(alice_id, 2);
    println!("BFS traversal from Alice (max depth 2): {:?}", bfs_result);

    // Test shortest path from Alice to Carol
    if let Some(path) = ontology.shortest_path(alice_id, carol_id) {
        println!("Shortest path from Alice to Carol: {:?}", path);
    } else {
        println!("No path found from Alice to Carol.");
    }

    Ok(())
}
