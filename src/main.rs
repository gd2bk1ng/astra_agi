// =============================================================================
//  Astra AGI - Main Entry Point with Warp Server
//  File: main.rs
//
//  Description: Runs Astra executor and starts Warp HTTP server for visualization API.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2026-01-11
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use astra_agi::runtime::executor::Executor;
use warp::Filter;
use serde_json::json;
use chrono::Utc;

#[tokio::main]
async fn main() {
    // Run your existing executor logic (synchronously)
    let mut executor = Executor::new();
    executor.execute("print('Hello, Astra AGI!')").unwrap();

    // Define Warp routes
    let learning_progress = warp::path!("api" / "visualization" / "learning_progress")
        .map(|| {
            warp::reply::json(&json!({
                "concepts_learned": 42,
                "research_sessions": 7,
                "code_modules_created": 3,
                "last_updated": Utc::now().to_rfc3339(),
            }))
        });

    let reasoning_chains = warp::path!("api" / "visualization" / "reasoning_chains")
        .map(|| {
            warp::reply::json(&json!({
                "Plan Generation": [
                    "Analyze goal",
                    "Generate options",
                    "Evaluate options",
                    "Select best plan",
                    "Execute plan"
                ],
                "Debugging": [
                    "Identify error",
                    "Hypothesize cause",
                    "Test fix",
                    "Verify resolution"
                ]
            }))
        });

    let routes = learning_progress.or(reasoning_chains);

    // Start warp server (this blocks)
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
