// =============================================================================
//  Astra AGI - Integration & Regression Tests
//  File: integration_tests.rs
//
//  Description:
//  Tests integration of Bayesian and Fuzzy reasoning, personality feedback,
//  intent management, and narrative memory coherence.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use astra_agi::runtime::Runtime;
use std::collections::{HashMap};

#[test]
fn test_full_epistemic_reasoning_cycle() {
    let mut runtime = Runtime::new();

    // Add Bayesian node
    runtime.epistemic_reasoner.bayesian.add_node(
        astra_agi::knowledge::bayesian_reasoner::BBNNode {
            id: 1,
            name: "TestFact".to_string(),
            parents: vec![],
            children: vec![],
            cpt: [(vec![], 0.6)].iter().cloned().collect(),
        }
    );

    let fact = astra_agi::knowledge::extended_ontology::Fact {
        subject: 0,
        predicate: "is".to_string(),
        object: "TestFact".to_string(),
        confidence: 0.6,
        provenance: astra_agi::knowledge::extended_ontology::Provenance::new("test_source", None),
    };

    // Incorporate evidence and check narrative memory log
    runtime.epistemic_reasoner.incorporate_evidence(&fact, true, &mut runtime.narrative_memory);

    let last_event = runtime.narrative_memory.events.back().unwrap();
    assert!(last_event.description.contains("Updated CPT"));

    // Test fuzzy logic AND
    let fuzzy_result = runtime.epistemic_reasoner.fuzzy_and(0.7, 0.8);
    assert!(fuzzy_result >= 0.7 && fuzzy_result <= 0.8);
}

#[test]
fn test_personality_feedback_loop() {
    let mut runtime = Runtime::new();

    let mut feedback = HashMap::new();
    feedback.insert("openness".to_string(), -0.3);
    feedback.insert("agreeableness".to_string(), 0.2);

    runtime.apply_personality_feedback(&feedback);

    assert!((runtime.personality.traits.openness - 0.5).abs() < 1e-6);
    assert!((runtime.personality.traits.agreeableness - 1.0).abs() < 1e-6);
}

#[test]
fn test_intent_priority_modification() {
    let mut runtime = Runtime::new();

    let intent_id = runtime.intent_manager.create_intent_with_metadata(
        "Test Intent",
        5,
        None,
    );

    let intent_before = runtime.intent_manager.get_intent(intent_id).unwrap().priority;

    // Simulate emotion and value states that boost priority
    runtime.emotion_state.urgency = 0.8;
    runtime.value_model.update_value("compassion", 0.9);

    let task_metadata = std::collections::HashMap::new();
    let modifier = astra_agi::emotion::compute_priority_modifier(
        &runtime.emotion_state,
        &runtime.value_model,
        &task_metadata,
    );

    let new_priority = ((intent_before as f32) * (1.0 + modifier)).max(0.0) as u32;
    runtime.intent_manager.update_intent(intent_id, Some(new_priority), None, None).unwrap();

    let intent_after = runtime.intent_manager.get_intent(intent_id).unwrap().priority;
    assert!(intent_after > intent_before);
}
