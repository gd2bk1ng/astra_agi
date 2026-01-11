// ============================================================================
//                          ASTRA AGI • API INTERFACE
//                 REST & WebSocket Gateway to Cognitive Runtime
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Serves as the primary external interaction layer for Astra AGI. This
//       module exposes REST and WebSocket endpoints that allow clients to
//       query system state, dispatch messages, and engage in real‑time
//       bidirectional communication with the cognitive engine.
//
//   Core Functions:
//       • Provide REST endpoints for structured queries and control actions
//       • Maintain WebSocket channels for streaming, events, and live updates
//       • Route inbound messages into the cognitive pipeline
//       • Expose safe, observable access to runtime state and activity
//
//   File:        /src/interfaces/api.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::runtime::Runtime;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub reply: String,
    pub emotion_state: String,
    pub personality_traits: String,
    pub recent_events: Vec<String>,
}

/// Astra API handler struct wrapping shared runtime instance.
#[derive(Clone)]
pub struct AstraApi {
    pub runtime: Arc<Mutex<Runtime>>,
}

impl AstraApi {
    /// Creates a new AstraApi instance with shared runtime.
    pub fn new(runtime: Arc<Mutex<Runtime>>) -> Self {
        Self { runtime }
    }

    /// Handles chat message POST requests asynchronously.
    pub async fn chat_handler(&self, req: web::Json<ChatRequest>) -> impl Responder {
        let mut runtime = self.runtime.lock().await;

        runtime.execute_program(&req.message).unwrap_or(());

        for _ in 0..5 {
            runtime.tick();
        }

        let personality = runtime.personality.clone();
        let reply = personality.respond_to_input(&req.message);

        let recent_events: Vec<String> = runtime
            .narrative_memory
            .recent_events(10)
            .iter()
            .map(|e| format!("[{}] {}: {}", e.timestamp, e.event_type, e.description))
            .collect();

        let response = ChatResponse {
            reply,
            emotion_state: format!("{:?}", runtime.emotion_state),
            personality_traits: format!("{:?}", personality.traits),
            recent_events,
        };

        HttpResponse::Ok().json(response)
    }
}
