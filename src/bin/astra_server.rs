// =============================================================================
//  Astra AGI - Backend Server
//  File: astra_server.rs
//
//  Description:
//  Web server exposing REST API for chat interaction with Astra AGI.
//  Allows sending messages, receiving Astra's replies, and viewing her internal states.
//  Integrates runtime, personality, emotion, and narrative memory subsystems.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use astra_agi::runtime::Runtime;

struct AppState {
    runtime: Mutex<Runtime>,
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    reply: String,
    emotion_state: String,
    personality_traits: String,
    recent_events: Vec<String>,
}

async fn chat_handler(data: web::Data<AppState>, req: web::Json<ChatRequest>) -> impl Responder {
    let mut runtime = data.runtime.lock().unwrap();

    // Execute user message as Astra program (or adapt as needed)
    runtime.execute_program(&req.message);

    // Run several ticks to process
    for _ in 0..5 {
        runtime.tick();
    }

    // Generate personality response
    let mut personality = runtime.personality.clone();
    let reply = personality.respond_to_input(&req.message);

    // Format recent narrative events for client
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let runtime = Runtime::new();

    println!("Starting Astra AGI Web Server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                runtime: Mutex::new(runtime.clone()),
            }))
            .route("/chat", web::post().to(chat_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
