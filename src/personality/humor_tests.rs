// =============================================================================
//  Astra AGI - Personality Humor Module Tests
//  File: humor_tests.rs
//
//  Author:      Alex Roussinov
//  Created:     2026-01-15
//  Updated:     2026-01-15
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emotion::EmotionState;
    use crate::personality::{Personality, PersonalityTraits};

    #[test]
    fn humor_style_selection_varies_with_traits_and_emotion() {
        let humor = Humor::new();

        let traits = PersonalityTraits {
            openness: 0.7,
            conscientiousness: 0.5,
            extraversion: 0.8,
            agreeableness: 0.6,
            neuroticism: 0.2,
        };

        let happy_emotion = EmotionState {
            happiness: 0.9,
            sadness: 0.1,
            anger: 0.0,
            fear: 0.0,
        };

        let style = humor.determine_style(&traits, &happy_emotion);
        assert!(matches!(style, HumorStyle::Playful | HumorStyle::Punny | HumorStyle::Dry));

        let sad_emotion = EmotionState {
            happiness: 0.1,
            sadness: 0.7,
            anger: 0.0,
            fear: 0.0,
        };

        let style_sad = humor.determine_style(&traits, &sad_emotion);
        assert_eq!(style_sad, HumorStyle::Lighthearted);
    }

    #[test]
    fn tell_joke_returns_non_empty_string() {
        let humor = Humor::new();
        let style = HumorStyle::Playful;
        let joke = humor.tell_joke(style);
        assert!(!joke.is_empty());
    }

    #[test]
    fn personality_responds_with_joke_on_trigger() {
        let traits = PersonalityTraits {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
        };
        let personality = Personality::new(traits);

        let emotion = EmotionState {
            happiness: 0.8,
            sadness: 0.1,
            anger: 0.0,
            fear: 0.0,
        };

        let input = "Tell me a joke!";
        let response = personality.respond_to_input(input, &emotion, Some("request_joke"));
        assert!(response.contains("joke"));
    }

    #[test]
    fn personality_responds_normally_without_joke_trigger() {
        let traits = PersonalityTraits {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
        };
        let personality = Personality::new(traits);

        let emotion = EmotionState {
            happiness: 0.8,
            sadness: 0.1,
            anger: 0.0,
            fear: 0.0,
        };

        let input = "Hello Astra";
        let response = personality.respond_to_input(input, &emotion, None);
        assert!(response.contains("Hello"));
    }
}
