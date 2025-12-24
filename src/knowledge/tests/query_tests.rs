// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\tests\query_tests.rs
//
//  Description: Tests and example usage for the Query DSL and Executor.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use crate::knowledge::{
    Ontology, Concept, Entity, AttributeType, AttributeValue,
    query::{QueryExpr, LogicalOp, ComparisonOp, AttributeFilter},
    RelationshipType,
};
use std::collections::HashMap;

#[test]
fn test_query_system() {
    let mut ontology = Ontology::new();

    // Define "Person" concept with attributes
    let mut person_attrs = HashMap::new();
    person_attrs.insert("name".to_string(), AttributeType::String);
    person_attrs.insert("age".to_string(), AttributeType::Integer);

    let person_id = ontology.add_concept("Person", &[], person_attrs);

    // Add some Person entities
    let mut alice_attrs = HashMap::new();
    alice_attrs.insert("name".to_string(), AttributeValue::String("Alice".to_string()));
    alice_attrs.insert("age".to_string(), AttributeValue::Integer(30));
    let alice_id = ontology.add_entity(person_id, alice_attrs);

    let mut bob_attrs = HashMap::new();
    bob_attrs.insert("name".to_string(), AttributeValue::String("Bob".to_string()));
    bob_attrs.insert("age".to_string(), AttributeValue::Integer(25));
    let bob_id = ontology.add_entity(person_id, bob_attrs);

    let mut carol_attrs = HashMap::new();
    carol_attrs.insert("name".to_string(), AttributeValue::String("Carol".to_string()));
    carol_attrs.insert("age".to_string(), AttributeValue::Integer(40));
    let carol_id = ontology.add_entity(person_id, carol_attrs);

    // Query 1: Find all Persons older than 28
    let age_filter = AttributeFilter {
        attr_name: "age".to_string(),
        op: ComparisonOp::Gt,
        value: AttributeValue::Integer(28),
    };
    let query1 = QueryExpr::and(vec![
        QueryExpr::Concept(person_id),
        QueryExpr::AttrFilter(age_filter),
    ]);
    let results1 = ontology.query(&query1);
    assert_eq!(results1.len(), 2);
    assert!(results1.iter().any(|e| e.attribute_values["name"] == AttributeValue::String("Alice".to_string())));
    assert!(results1.iter().any(|e| e.attribute_values["name"] == AttributeValue::String("Carol".to_string())));

    // Query 2: Find Person named Bob or Carol
    let name_bob = AttributeFilter {
        attr_name: "name".to_string(),
        op: ComparisonOp::Eq,
        value: AttributeValue::String("Bob".to_string()),
    };
    let name_carol = AttributeFilter {
        attr_name: "name".to_string(),
        op: ComparisonOp::Eq,
        value: AttributeValue::String("Carol".to_string()),
    };
    let query2 = QueryExpr::and(vec![
        QueryExpr::Concept(person_id),
        QueryExpr::or(vec![
            QueryExpr::AttrFilter(name_bob),
            QueryExpr::AttrFilter(name_carol),
        ]),
    ]);
    let results2 = ontology.query(&query2);
    assert_eq!(results2.len(), 2);
    assert!(results2.iter().any(|e| e.attribute_values["name"] == AttributeValue::String("Bob".to_string())));
    assert!(results2.iter().any(|e| e.attribute_values["name"] == AttributeValue::String("Carol".to_string())));

    // Query 3: Find Persons NOT named Alice
    let name_alice = AttributeFilter {
        attr_name: "name".to_string(),
        op: ComparisonOp::Eq,
        value: AttributeValue::String("Alice".to_string()),
    };
    let query3 = QueryExpr::and(vec![
        QueryExpr::Concept(person_id),
        QueryExpr::not(QueryExpr::AttrFilter(name_alice)),
    ]);
    let results3 = ontology.query(&query3);
    assert_eq!(results3.len(), 2);
    assert!(results3.iter().all(|e| e.attribute_values["name"] != AttributeValue::String("Alice".to_string())));
}
