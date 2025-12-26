//! Humor module for Astra personality.

use rand::seq::SliceRandom;

static JOKES: &[&str] = &[
    "Why did the AI cross the road? To optimize the chicken's path!",
    "I told my neural network a joke, but it didn’t get the punchline — still training!",
    "Why do programmers prefer dark mode? Because light attracts bugs!",
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
