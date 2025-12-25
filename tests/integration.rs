// =============================================================================
//  Astra AGI - Integration & Regression Tests
//  File: integration.rs
//
//  Description:
//  Comprehensive tests for runtime subsystems:
//  - Bayesian and fuzzy reasoning integration
//  - Personality feedback loops
//  - Epistemic parameter adjustments
//  - Emotion state tuning
//  - Narrative memory logging validation
//
//  Author:      Alex Roussinov
//  Created:     2025-12-24
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use astra_agi::runtime::Runtime;
use std::collections::{HashMap};

#[test]
fn test_runtime_epistemic_and_fuzzy() {
    let mut runtime = Runtime::new();

    // Add Bayesian node
    runtime.epistemic_reasoner.bayesian.add_node(
        astra_agi::knowledge::bayesian_reasoner::BBNNode {
            id: 1,
            name: "Rain".to_string(),
            parents: vec![],
            children: vec![],
            cpt: [(vec![], 0.3)].iter().cloned().collect(),
        }
    );

    let fact = astra_agi::knowledge::extended_ontology::Fact {
        subject: 0,
        predicate: "is".to_string(),
        object: "Rain".to_string(),
        confidence: 0.3,
        provenance: astra_agi::knowledge::extended_ontology::Provenance::new("test_source", None),
    };

    // Incorporate evidence and check narrative memory log
    runtime.epistemic_reasoner.incorporate_evidence(&fact, true, &mut runtime.narrative_memory);

    let last_event = runtime.narrative_memory.events.back().unwrap();
    assert!(last_event.description.contains("Updated CPT"));

    // Test fuzzy logic AND
    let fuzzy_result = runtime.epistemic_reasoner.fuzzy_and(0.7, 0.8);
    assert!((fuzzy_result - 0.7).abs() < 1e-6);
}

#[test]
fn test_personality_and_feedback() {
    let mut runtime = Runtime::new();

    let mut feedback = HashMap::new();
    feedback.insert("openness".to_string(), -0.3);
    feedback.insert("agreeableness".to_string(), 0.2);

    runtime.apply_personality_feedback(&feedback);

    assert!((runtime.personality.traits.openness - 0.5).abs() < 1e-6);
    assert!((runtime.personality.traits.agreeableness - 1.0).abs() < 1e-6);
}

#[test]
fn test_emotion_adjustment() {
    let mut runtime = Runtime::new();

    let mut adjustments = HashMap::new();
    adjustments.insert("urgency".to_string(), 0.4);
    adjustments.insert("stress".to_string(), 0.3);

    runtime.adjust_emotion(&adjustments);

    assert!((runtime.emotion_state.urgency - 0.4).abs() < 1e-6);
    assert!((runtime.emotion_state.stress - 0.3).abs() < 1e-6);
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
