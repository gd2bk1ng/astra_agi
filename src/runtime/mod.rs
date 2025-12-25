// =============================================================================
//  Astra AGI - Executor Runtime Root
//  File: mod.rs
//
//  Description:
//  Core runtime orchestrator extended with Emotion and Value Models for affective task prioritization,
//  Intent Manager integration, and Narrative Memory for persistent event logging.
//  This enables Astra to behave as a living, adaptive system with emotional and ethical awareness.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-22
//  Updated:     2025-12-35
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

pub mod intent_manager;
pub mod executor;
pub mod scheduler;

use crate::emotion::{EmotionState, ValueModel};
use crate::memory::narrative_memory::NarrativeMemory;
use crate::runtime::intent_manager::IntentManager;
use executor::Executor;
use scheduler::Scheduler;

/// Orchestrates the execution of Astra programs.
/// Manages executor and scheduler lifecycles and coordinates ticks.
/// Integrates affective models and narrative memory for advanced AGI behavior.
pub struct Runtime {
    executor: Executor,
    scheduler: Scheduler,
    intent_manager: IntentManager,
    emotion_state: EmotionState,
    value_model: ValueModel,
    narrative_memory: NarrativeMemory,
}

impl Runtime {
    /// Creates a new Runtime instance.
    pub fn new() -> Self {
        Runtime {
            executor: Executor::new(),
            scheduler: Scheduler::new(),
            intent_manager: IntentManager::new(),
            emotion_state: EmotionState::new(),
            value_model: ValueModel::new(),
            narrative_memory: NarrativeMemory::new(1000), // Capacity for 1000 events
        }
    }

    /// Starts the runtime components.
    pub fn start(&mut self) {
        self.scheduler.start();
        self.executor.start();
        self.narrative_memory.add_event("runtime_start", "Runtime started", None);
    }

    /// Parses and executes Astra source code.
    pub fn execute_program(&mut self, program: &str) {
        self.narrative_memory.add_event("program_execution", format!("Executing program: {}", program), None);
        let ast = self.executor.parse(program).expect("Parsing failed");
        self.executor.execute(&ast);
        // Create an intent for this program execution
        let intent_id = self.intent_manager.create_intent("Program execution intent", 10);
        self.narrative_memory.add_event("intent_created", format!("Intent {} created", intent_id), None);
    }

    /// Advances runtime by one tick.
    pub fn tick(&mut self) {
        // Update emotion state based on workload and deadlines
        let mut stimuli = std::collections::HashMap::new();
        let next_intent = self.intent_manager.next_intent();
        if let Some(intent) = &next_intent {
            // Example: urgency based on deadline proximity
            if let Some(deadline) = intent.deadline {
                let now = std::time::Instant::now();
                let duration_to_deadline = deadline.saturating_duration_since(now);
                let urgency = 1.0 - (duration_to_deadline.as_secs_f32() / 3600.0).clamp(0.0, 1.0); // 1 hour window
                stimuli.insert("deadline_proximity".to_string(), urgency);
            }
        }
        stimuli.insert("workload".to_string(), (self.intent_manager.all_intents().len() as f32 / 100.0).clamp(0.0, 1.0));
        self.emotion_state.update(&stimuli);

        // Modify intent priority based on emotion and values
        if let Some(intent) = next_intent {
            let task_metadata = std::collections::HashMap::new(); // Extend to pass real metadata
            let modifier = crate::emotion::compute_priority_modifier(&self.emotion_state, &self.value_model, &task_metadata);
            let new_priority = ((intent.priority as f32) * (1.0 + modifier)).max(0.0) as u32;
            self.intent_manager.update_intent(intent.id, Some(new_priority), None, None).unwrap_or_else(|e| {
                self.narrative_memory.add_event("error", format!("Failed to update intent priority: {}", e), None);
            });
        }

        self.scheduler.tick();
        self.executor.tick();

        self.narrative_memory.add_event("tick", "Runtime tick completed", None);
    }
}
