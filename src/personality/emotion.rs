// ============================================================================
//                         ASTRA AGI • EMOTION SUBSYSTEM
//        Affective State Modeling, Mood Dynamics & Stimulus Integration
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements Astra’s affective processing layer, providing a structured
//       representation of emotional state, mood evolution, and dynamic
//       modulation in response to internal and external stimuli. This subsystem
//       supports expressive behavior, adaptive personality responses, and
//       emotionally‑aware interaction patterns.
//
//   Core Functions:
//       • Represent multidimensional emotional state (happiness, sadness, anger, fear)
//       • Compute affective valence and arousal for behavioral modulation
//       • Blend emotional states using weighted stimuli
//       • Apply decay functions for natural emotional stabilization
//       • Provide mood tracking for long‑term affective continuity
//
//   File:        /src/personality/emotion.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use std::time::{Duration, Instant};

/// Represents Astra’s instantaneous emotional state.
/// Values are normalized between 0.0 and 1.0.
#[derive(Debug, Clone)]
pub struct EmotionState {
    pub happiness: f32,
    pub sadness: f32,
    pub anger: f32,
    pub fear: f32,
}

impl EmotionState {
    /// Creates a neutral emotional state.
    pub fn neutral() -> Self {
        Self {
            happiness: 0.5,
            sadness: 0.0,
            anger: 0.0,
            fear: 0.0,
        }
    }

    /// Computes emotional valence: positive vs. negative affect.
    pub fn valence(&self) -> f32 {
        self.happiness - (self.sadness + self.anger + self.fear) * 0.5
    }

    /// Computes emotional arousal: intensity of activation.
    pub fn arousal(&self) -> f32 {
        (self.happiness + self.anger + self.fear).max(0.1)
    }

    /// Blends this emotional state with another using a weight factor.
    pub fn blend(&mut self, other: &EmotionState, weight: f32) {
        let w = weight.clamp(0.0, 1.0);
        self.happiness = self.happiness * (1.0 - w) + other.happiness * w;
        self.sadness = self.sadness * (1.0 - w) + other.sadness * w;
        self.anger = self.anger * (1.0 - w) + other.anger * w;
        self.fear = self.fear * (1.0 - w) + other.fear * w;
    }
}

/// Represents long‑term mood, which evolves more slowly than emotion.
#[derive(Debug, Clone)]
pub struct Mood {
    pub baseline: f32, // 0 (negative) to 1 (positive)
}

impl Mood {
    pub fn new() -> Self {
        Self { baseline: 0.6 }
    }

    /// Updates mood based on emotional valence.
    pub fn update_from_emotion(&mut self, emotion: &EmotionState) {
        let delta = emotion.valence() * 0.05;
        self.baseline = (self.baseline + delta).clamp(0.0, 1.0);
    }
}

/// Engine for managing emotional transitions, decay, and stimulus integration.
pub struct EmotionDynamics {
    pub current: EmotionState,
    pub mood: Mood,
    last_update: Instant,
    decay_rate: f32,
}

impl EmotionDynamics {
    pub fn new() -> Self {
        Self {
            current: EmotionState::neutral(),
            mood: Mood::new(),
            last_update: Instant::now(),
            decay_rate: 0.15,
        }
    }

    /// Applies natural emotional decay over time.
    fn apply_decay(&mut self) {
        let elapsed = self.last_update.elapsed().as_secs_f32();
        let decay_factor = (1.0 - self.decay_rate * elapsed).clamp(0.0, 1.0);

        self.current.happiness *= decay_factor;
        self.current.sadness *= decay_factor;
        self.current.anger *= decay_factor;
        self.current.fear *= decay_factor;

        self.last_update = Instant::now();
    }

    /// Applies an emotional stimulus (positive or negative).
    pub fn apply_stimulus(&mut self, stimulus: &EmotionState, intensity: f32) {
        self.apply_decay();
        self.current.blend(stimulus, intensity.clamp(0.0, 1.0));
        self.mood.update_from_emotion(&self.current);
    }

    /// Returns a snapshot of the current emotional state.
    pub fn snapshot(&self) -> EmotionState {
        self.current.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_blending() {
        let mut e1 = EmotionState::neutral();
        let e2 = EmotionState {
            happiness: 1.0,
            sadness: 0.0,
            anger: 0.0,
            fear: 0.0,
        };

        e1.blend(&e2, 0.5);
        assert!(e1.happiness > 0.7);
    }

    #[test]
    fn test_emotion_dynamics_stimulus() {
        let mut dynamics = EmotionDynamics::new();
        let positive = EmotionState {
            happiness: 1.0,
            sadness: 0.0,
            anger: 0.0,
            fear: 0.0,
        };

        dynamics.apply_stimulus(&positive, 0.8);
        assert!(dynamics.current.happiness > 0.7);
        assert!(dynamics.mood.baseline > 0.6);
    }
}
