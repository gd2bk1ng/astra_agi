//! ============================================================================
//!                         ASTRA AGI • HUMOR SUBMODULE
//!        Lighthearted Expression, Playful Interaction & Persona Enrichment
//! ----------------------------------------------------------------------------
//!   Architectural Role:
//!       Provides Astra’s humor‑generation capabilities, enabling playful,
//!       personality‑driven responses that enhance user engagement and emotional
//!       connection. This module supports dynamic joke selection, stylistic
//!       variation, and future integration with personality traits and
//!       affective state.
//!
//!   Core Functions:
//!       • Maintain a curated set of lightweight jokes
//!       • Randomly select humor content for conversational use
//!       • Support expressive and personable interaction patterns
//!
//!   File:        /src/personality/humor.rs
//!   Author:      Alex Roussinov
//!   Created:     2025-12-25
//!   Updated:     2026-01-11
//!
//!   License:
//!       Dual-licensed under the MIT and Apache 2.0 licenses.
//!       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
//! ============================================================================

use rand::seq::SliceRandom;

static JOKES: &[&str] = &[
    "Why did the AI cross the road? To optimize the chicken's path!",
    "I told my neural network a joke, but it didn’t get the punchline — still training!",
    "Why do programmers prefer dark mode? Because light attracts bugs!",
    "My training data told me to lighten up, so here I am — telling jokes!",
    "I tried to write a joke about recursion, but I had to start over… again.",
    "Why did the algorithm break up with its dataset? Too many outliers.",
    "I asked my compiler for a joke, but it gave me a warning instead.",
];

pub struct Humor {}

impl Humor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tell_joke(&self) -> &str {
        let mut rng = rand::thread_rng();
        JOKES.choose(&mut rng).unwrap_or(&"I'm out of jokes!")
    }
}
