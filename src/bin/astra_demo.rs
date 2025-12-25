// =============================================================================
//  Astra AGI - Interactive Demo CLI
//  File: astra_demo.rs
//
//  Description:
//  Demonstrates Astra AGI running programs, reasoning with Bayesian and fuzzy logic,
//  exhibiting personality traits, and logging narrative memory.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use std::io::{self, Write};
use astra_agi::runtime::Runtime;
use astra_agi::personality::personality::Personality;

fn main() {
    let mut runtime = Runtime::new();
    runtime.start();

    let mut personality = Personality::new();

    println!("Welcome to Astra AGI Demo. Type 'exit' to quit.");

    loop {
        print!("astra> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        runtime.execute_program(input);

        for _ in 0..5 {
            runtime.tick();
        }

        println!("Emotion State: {:?}", runtime.emotion_state);
        println!("Personality response: {}", personality.respond_to_input(input));

        let recent_events = runtime.narrative_memory.recent_events(5);
        println!("Recent Narrative Events:");
        for event in recent_events {
            println!(" - [{}] {}: {}", event.timestamp, event.event_type, event.description);
        }
    }

    println!("Goodbye!");
}
