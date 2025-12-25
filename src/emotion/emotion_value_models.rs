// =============================================================================
//  Astra AGI - Emotion and Value Models (EVM)
//  File: emotion_value_models.rs
//
//  Description:
//  Implements affective state modeling and value-based decision influences.
//  Simulates emotions such as urgency, motivation, and stress,
//  and represents core values to guide ethical and sustainable AGI behavior.
//
//  These models modulate task prioritization and reasoning,
//  giving Astra a richer, more human-aligned decision framework.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-24
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use std::collections::HashMap;

/// Represents core affective/emotional states influencing behavior.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmotionState {
    /// Urgency level: 0.0 (none) to 1.0 (max)
    pub urgency: f32,
    /// Motivation level: 0.0 (none) to 1.0 (max)
    pub motivation: f32,
    /// Stress level: 0.0 (none) to 1.0 (max)
    pub stress: f32,
}

impl EmotionState {
    /// Creates a new EmotionState with default neutral values.
    pub fn new() -> Self {
        EmotionState {
            urgency: 0.0,
            motivation: 0.5,
            stress: 0.0,
        }
    }

    /// Updates the emotion state based on external stimuli or internal feedback.
    /// Example: increase urgency if deadline is near.
    pub fn update(&mut self, stimuli: &HashMap<String, f32>) {
        if let Some(&val) = stimuli.get("deadline_proximity") {
            self.urgency = self.urgency.max(val);
        }
        if let Some(&val) = stimuli.get("task_importance") {
            self.motivation = self.motivation.max(val);
        }
        if let Some(&val) = stimuli.get("workload") {
            self.stress = self.stress.max(val);
        }

        // Clamp values to [0.0, 1.0]
        self.urgency = self.urgency.clamp(0.0, 1.0);
        self.motivation = self.motivation.clamp(0.0, 1.0);
        self.stress = self.stress.clamp(0.0, 1.0);
    }
}

/// Represents Astra's core values influencing ethical and sustainable behavior.
#[derive(Debug, Clone)]
pub struct ValueModel {
    /// Value weights for different principles (0.0 to 1.0)
    pub values: HashMap<String, f32>,
}

impl ValueModel {
    /// Creates a ValueModel with default core values.
    pub fn new() -> Self {
        let mut values = HashMap::new();
        values.insert("compassion".to_string(), 1.0);
        values.insert("integrity".to_string(), 1.0);
        values.insert("sustainability".to_string(), 1.0);
        values.insert("dignity".to_string(), 1.0);
        ValueModel { values }
    }

    /// Updates a value weight.
    pub fn update_value(&mut self, key: &str, weight: f32) {
        if let Some(v) = self.values.get_mut(key) {
            *v = weight.clamp(0.0, 1.0);
        }
    }

    /// Retrieves the weight of a specific value.
    pub fn get_value(&self, key: &str) -> Option<f32> {
        self.values.get(key).copied()
    }
}

/// Combines emotion and value states to compute a task priority modifier.
/// This modifier can increase or decrease task priority based on affective and ethical factors.
pub fn compute_priority_modifier(emotion: &EmotionState, values: &ValueModel, task_metadata: &HashMap<String, f32>) -> f32 {
    // Simple weighted sum example:
    let urgency_weight = 0.6;
    let motivation_weight = 0.3;
    let stress_weight = -0.2; // Stress reduces priority

    let compassion = values.get_value("compassion").unwrap_or(1.0);
    let integrity = values.get_value("integrity").unwrap_or(1.0);

    let compassion_weight = 0.4;
    let integrity_weight = 0.4;

    let mut modifier = 0.0;
    modifier += urgency_weight * emotion.urgency;
    modifier += motivation_weight * emotion.motivation;
    modifier += stress_weight * emotion.stress;

    // Example: if task metadata has "ethical_importance", amplify by values
    if let Some(&ethical_importance) = task_metadata.get("ethical_importance") {
        modifier += compassion_weight * compassion * ethical_importance;
        modifier += integrity_weight * integrity * ethical_importance;
    }

    modifier.clamp(-1.0, 1.0) // Modifier range [-1, 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_update() {
        let mut emotion = EmotionState::new();
        let mut stimuli = HashMap::new();
        stimuli.insert("deadline_proximity".to_string(), 0.8);
        stimuli.insert("workload".to_string(), 0.5);
        emotion.update(&stimuli);
        assert!(emotion.urgency >= 0.8);
        assert!(emotion.stress >= 0.5);
    }

    #[test]
    fn test_value_model_update_and_get() {
        let mut values = ValueModel::new();
        values.update_value("compassion", 0.7);
        assert_eq!(values.get_value("compassion"), Some(0.7));
    }

    #[test]
    fn test_priority_modifier_computation() {
        let emotion = EmotionState {
            urgency: 0.9,
            motivation: 0.5,
            stress: 0.1,
        };
        let values = ValueModel::new();

        let mut task_metadata = HashMap::new();
        task_metadata.insert("ethical_importance".to_string(), 0.8);

        let modifier = compute_priority_modifier(&emotion, &values, &task_metadata);
        assert!(modifier > 0.0);
    }
}
