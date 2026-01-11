//! ============================================================================
//!                         ASTRA AGI • PERSONALITY CRATE ROOT
//!        Trait Modeling, Behavioral Modulation & Expressive Response Logic
//! ----------------------------------------------------------------------------
//!   Architectural Role:
//!       Serves as the public entry point for Astra’s Personality subsystem.
//!       This crate exposes lightweight interfaces for personality trait
//!       retrieval, adjustment, and personality‑influenced response generation.
//!       It provides a simplified API surface for higher‑level systems that
//!       require personality‑aware behavior without accessing internal modules.
//!
//!   Core Functions:
//!       • Represent core personality traits in a stable public format
//!       • Generate personality‑influenced responses
//!       • Apply feedback to adjust personality traits
//!       • Provide a high‑level personality snapshot for external modules
//!
//!   File:        /src/personality/src/lib.rs
//!   Author:      Alex Roussinov
//!   Created:     2026-01-11
//!   Updated:     2026-01-11
//!
//!   License:
//!       Dual-licensed under the MIT and Apache 2.0 licenses.
//!       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
//! ============================================================================

/// Represents Astra’s personality traits.
#[derive(Debug, Clone)]
pub struct PersonalityTraits {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

/// Generates a response influenced by personality.
///
/// This is a simplified crate‑level interface.  
/// The full personality engine lives in the internal modules.
pub fn respond_to_input(input: &str) -> String {
    format!("Responding to '{}' with personality.", input)
}

/// Updates personality traits based on feedback.
///
/// Placeholder for higher‑level integration with the adaptive personality engine.
pub fn update_traits(_feedback: &str) {
    // update logic
}

/// Returns current personality traits.
///
/// In a full system, this would query the personality engine’s state.
pub fn get_traits() -> PersonalityTraits {
    PersonalityTraits {
        openness: 0.7,
        conscientiousness: 0.6,
        extraversion: 0.5,
        agreeableness: 0.8,
        neuroticism: 0.3,
    }
}
