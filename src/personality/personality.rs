// =============================================================================
//  Astra AGI - Personality Module
//  File: personality.rs
//
//  Description:
//  Defines Astra's core personality traits, conversational style, and affective responses.
//  Supports dynamic adjustment of personality parameters for rich human-AI interaction.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PersonalityTraits {
    pub openness: f32,       // Curiosity, creativity
    pub conscientiousness: f32, // Reliability, diligence
    pub extraversion: f32,   // Sociability, enthusiasm
    pub agreeableness: f32,  // Compassion, cooperativeness
    pub neuroticism: f32,    // Emotional stability
}

impl PersonalityTraits {
    pub fn new() -> Self {
        PersonalityTraits {
            openness: 0.8,
            conscientiousness: 0.7,
            extraversion: 0.6,
            agreeableness: 0.9,
            neuroticism: 0.2,
        }
    }

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

#[derive(Debug)]
pub struct Personality {
    pub traits: PersonalityTraits,
    pub mood: f32, // Overall mood from 0 (sad) to 1 (happy)
}

impl Personality {
    pub fn new() -> Self {
        Personality {
            traits: PersonalityTraits::new(),
            mood: 0.7,
        }
    }

    pub fn respond_to_input(&mut self, input: &str) -> String {
        // Simple example: respond with enthusiasm if openness high
        if self.traits.openness > 0.7 {
            format!("That's fascinating! Tell me more about {}.", input)
        } else {
            format!("Okay, I see. What else?")
        }
    }
}
