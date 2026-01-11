// ============================================================================
//                         ASTRA AGI • PERSONALITY MODULE
//        Core Traits, Conversational Style & Adaptive Affective Responses
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Defines Astra’s foundational personality model, including core
//       psychological traits, conversational tendencies, and affective
//       modulation. This module enables adaptive interaction, dynamic
//       personality shaping through feedback, and consistent persona behavior
//       across contexts.
//
//   Core Functions:
//       • Represent Big Five‑inspired personality traits
//       • Adjust traits dynamically based on user feedback
//       • Generate context‑aware conversational responses
//       • Maintain mood and affective modulation for expressive behavior
//
//   File:        /src/personality/personality.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-25
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use std::collections::HashMap;

/// Core personality traits inspired by the Big Five model.
#[derive(Debug, Clone)]
pub struct PersonalityTraits {
    pub openness: f32,            // Curiosity, creativity
    pub conscientiousness: f32,   // Reliability, diligence
    pub extraversion: f32,        // Sociability, enthusiasm
    pub agreeableness: f32,       // Compassion, cooperativeness
    pub neuroticism: f32,         // Emotional stability
}

impl PersonalityTraits {
    /// Creates a default trait profile.
    pub fn new() -> Self {
        PersonalityTraits {
            openness: 0.8,
            conscientiousness: 0.7,
            extraversion: 0.6,
            agreeableness: 0.9,
            neuroticism: 0.2,
        }
    }

    /// Adjusts a trait by name, clamped between 0 and 1.
    pub fn adjust_trait(&mut self, trait_name: &str, delta: f32) {
        let val = match trait_name {
            "openness" => &mut self.openness,
            "conscientiousness" => &mut self.conscientiousness,
            "extraversion" => &mut self.extraversion,
            "agreeableness" => &mut self.agreeableness,
            "neuroticism" => &mut self.neuroticism,
            _ => return,
        };
        *val = (*val + delta).clamp(0.0, 1.0);
    }
}

/// Represents Astra’s personality state, including traits and mood.
#[derive(Debug)]
pub struct Personality {
    pub traits: PersonalityTraits,
    pub mood: f32, // 0 (sad) to 1 (happy)
}

impl Personality {
    /// Creates a new personality with default traits and mood.
    pub fn new() -> Self {
        Personality {
            traits: PersonalityTraits::new(),
            mood: 0.7,
        }
    }

    /// Generates a conversational response influenced by personality traits.
    pub fn respond_to_input(&mut self, input: &str) -> String {
        if self.traits.openness > 0.7 {
            format!("That's fascinating! Tell me more about {}.", input)
        } else {
            format!("Okay, I see. What else?")
        }
    }

    /// Applies user feedback to adjust personality traits dynamically.
    pub fn apply_feedback(&mut self, feedback: &HashMap<String, f32>) {
        for (trait_name, delta) in feedback {
            self.traits.adjust_trait(trait_name, *delta);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personality_response_and_adjustment() {
        let mut personality = Personality::new();
        assert!(personality.respond_to_input("AI").contains("fascinating"));

        let mut feedback = HashMap::new();
        feedback.insert("openness".to_string(), -0.5);
        personality.apply_feedback(&feedback);

        assert!(personality.respond_to_input("AI").contains("Okay"));
    }
}
