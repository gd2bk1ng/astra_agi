// =============================================================================
//  Astra AGI - Executor Runtime Root
//  File: mod.rs
//
//  Description:
//  Core runtime orchestrator extended with Emotion and Value Models for affective task prioritization,
//  Intent Manager integration, Personality module, Narrative Memory for persistent event logging,
//  and Advanced Epistemic Reasoner integration.
//  This enables Astra to behave as a living, adaptive system with emotional and ethical awareness.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

pub mod executor;
pub mod scheduler;
pub mod intent_manager;

use crate::emotion::{EmotionState, ValueModel};
use crate::memory::narrative_memory::NarrativeMemory;
use crate::personality::personality::Personality;
use crate::knowledge::advanced_epistemic::AdvancedEpistemicReasoner;

use executor::Executor;
use scheduler::Scheduler;
use intent_manager::IntentManager;

/// The main runtime struct integrating all subsystems.
pub struct Runtime {
    pub executor: Executor,
    pub scheduler: Scheduler,
    pub intent_manager: IntentManager,
    pub emotion_state: EmotionState,
    pub value_model: ValueModel,
    pub personality: Personality,
    pub narrative_memory: NarrativeMemory,
    pub epistemic_reasoner: AdvancedEpistemicReasoner,
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
            personality: Personality::new(),
            narrative_memory: NarrativeMemory::new(1000),
            epistemic_reasoner: AdvancedEpistemicReasoner::new(),
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
            if let Some(deadline) = intent.deadline {
                let now = std::time::Instant::now();
                let duration_to_deadline = deadline.saturating_duration_since(now);
                let urgency = 1.0 - (duration_to_deadline.as_secs_f32() / 3600.0).clamp(0.0, 1.0);
                stimuli.insert("deadline_proximity".to_string(), urgency);
            }
        }
        stimuli.insert("workload".to_string(), (self.intent_manager.all_intents().len() as f32 / 100.0).clamp(0.0, 1.0));
        self.emotion_state.update(&stimuli);

        // Modify intent priority based on emotion and values
        if let Some(intent) = next_intent {
            let task_metadata = std::collections::HashMap::new(); // Extend as needed
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

    /// Adjusts personality traits based on user feedback.
    ///
    /// # Arguments
    /// * `feedback` - Map of trait names to adjustment deltas (-1.0 to 1.0).
    pub fn apply_personality_feedback(&mut self, feedback: &std::collections::HashMap<String, f32>) {
        self.personality.apply_feedback(feedback);
        self.narrative_memory.add_event(
            "personality_feedback",
            format!("Applied personality feedback: {:?}", feedback),
            None,
        );
    }

    /// Adjusts epistemic parameters dynamically.
    ///
    /// # Arguments
    /// * `params` - Map of parameter names to new values.
    pub fn adjust_epistemic_parameters(&mut self, params: &std::collections::HashMap<String, f64>) {
        for (key, value) in params {
            self.epistemic_reasoner.parameters.insert(key.clone(), *value);
        }
        self.narrative_memory.add_event(
            "epistemic_parameters_adjusted",
            format!("Updated parameters: {:?}", params),
            None,
        );
    }

    /// Applies feedback to emotion state parameters.
    ///
    /// # Arguments
    /// * `adjustments` - Map of emotion parameter names ("urgency", "motivation", "stress") to adjustment deltas.
    pub fn adjust_emotion(&mut self, adjustments: &std::collections::HashMap<String, f32>) {
        for (key, delta) in adjustments {
            match key.as_str() {
                "urgency" => {
                    self.emotion_state.urgency = (self.emotion_state.urgency + delta).clamp(0.0, 1.0);
                }
                "motivation" => {
                    self.emotion_state.motivation = (self.emotion_state.motivation + delta).clamp(0.0, 1.0);
                }
                "stress" => {
                    self.emotion_state.stress = (self.emotion_state.stress + delta).clamp(0.0, 1.0);
                }
                _ => {}
            }
        }
        self.narrative_memory.add_event("emotion_adjusted", format!("Emotion adjusted: {:?}", self.emotion_state), None);
    }
}
