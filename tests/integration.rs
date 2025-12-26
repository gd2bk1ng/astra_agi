// =============================================================================
// Astra AGI - Integration & Regression Tests
// File: integration.rs
//
// Description:
// Comprehensive integration tests for Astra AGI runtime and interfaces,
// including Bayesian reasoning, fuzzy logic, personality feedback, narrative memory,
// and API endpoint simulation.
//
// Author:      Alex Roussinov
// Created:     2025-12-24
// Updated:     2025-12-26
//
// Licensed under MIT OR Apache 2.0
// =============================================================================

use astra_agi::runtime::Runtime;
use astra_agi::interfaces::api::AstraApi;
use actix_web::{test, App};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn test_runtime_epistemic_and_fuzzy() {
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

    runtime.epistemic_reasoner.incorporate_evidence(&fact, true, &mut runtime.narrative_memory);

    let last_event = runtime.narrative_memory.events.back().unwrap();
    assert!(last_event.description.contains("Updated CPT"));

    let fuzzy_result = runtime.epistemic_reasoner.fuzzy_and(0.7, 0.8);
    assert!((fuzzy_result - 0.7).abs() < 1e-6);
}

#[tokio::test]
async fn test_personality_and_feedback() {
    let mut runtime = Runtime::new();

    let response_before = runtime.personality.respond_to_input("Hello Astra");
    runtime.personality.update_traits("positive feedback");
    let response_after = runtime.personality.respond_to_input("Hello Astra");

    assert_ne!(response_before, response_after);
}

#[actix_rt::test]
async fn test_api_chat_endpoint() {
    let runtime = Arc::new(Mutex::new(Runtime::new()));
    let api = AstraApi::new(runtime.clone());

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(api))
            .route("/chat", actix_web::web::post().to(AstraApi::chat_handler))
    ).await;

    let req = test::TestRequest::post()
        .uri("/chat")
        .set_json(&serde_json::json!({"message": "Hello Astra"}))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("reply"));
}
